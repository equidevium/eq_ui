//! Export generators for EqGrid bulk data export.
//!
//! Supports CSV, JSON, TXT (tab-separated), and ODS (OpenDocument Spreadsheet).
//! The ODS generator includes a minimal ZIP builder and CRC32 implementation,
//! keeping the crate dependency-free.

use super::column_def::EqColumnDef;

// ── Public API ─────────────────────────────────────────────────────

/// Generate CSV content from the selected rows.
///
/// Produces RFC 4180-compliant CSV: fields containing commas, quotes,
/// or newlines are quoted, and embedded quotes are doubled.
pub fn export_csv<T: Clone + PartialEq>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    indices: &[usize],
) -> String {
    let mut out = String::new();

    // Header row
    for (i, col) in columns.iter().enumerate() {
        if i > 0 { out.push(','); }
        csv_escape(&mut out, col.header);
    }
    out.push('\n');

    // Data rows
    for &idx in indices {
        let row = &data[idx];
        for (i, col) in columns.iter().enumerate() {
            if i > 0 { out.push(','); }
            let val = (col.value_getter)(row);
            csv_escape(&mut out, &val);
        }
        out.push('\n');
    }

    out
}

/// Generate JSON content from the selected rows.
///
/// Produces a JSON array of objects, each keyed by column ID.
pub fn export_json<T: Clone + PartialEq>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    indices: &[usize],
) -> String {
    let mut out = String::from("[\n");

    for (row_idx, &idx) in indices.iter().enumerate() {
        let row = &data[idx];
        out.push_str("  {");

        for (col_idx, col) in columns.iter().enumerate() {
            if col_idx > 0 { out.push_str(", "); }
            let val = (col.value_getter)(row);
            out.push('"');
            json_escape(&mut out, col.id);
            out.push_str("\": \"");
            json_escape(&mut out, &val);
            out.push('"');
        }

        out.push('}');
        if row_idx + 1 < indices.len() { out.push(','); }
        out.push('\n');
    }

    out.push(']');
    out
}

/// Generate tab-separated plain text from the selected rows.
pub fn export_txt<T: Clone + PartialEq>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    indices: &[usize],
) -> String {
    let mut out = String::new();

    // Header row
    for (i, col) in columns.iter().enumerate() {
        if i > 0 { out.push('\t'); }
        out.push_str(col.header);
    }
    out.push('\n');

    // Data rows
    for &idx in indices {
        let row = &data[idx];
        for (i, col) in columns.iter().enumerate() {
            if i > 0 { out.push('\t'); }
            let val = (col.value_getter)(row);
            // Replace tabs/newlines to keep single-line cells.
            let clean: String = val.chars().map(|c| match c {
                '\t' => ' ',
                '\n' | '\r' => ' ',
                other => other,
            }).collect();
            out.push_str(&clean);
        }
        out.push('\n');
    }

    out
}

/// Generate ODS (OpenDocument Spreadsheet) bytes from the selected rows.
///
/// Returns raw bytes of a valid `.ods` file that can be opened by
/// LibreOffice, Google Sheets, and other OpenDocument-compatible apps.
pub fn export_ods<T: Clone + PartialEq>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    indices: &[usize],
) -> Vec<u8> {
    let mimetype = b"application/vnd.oasis.opendocument.spreadsheet";
    let content_xml = build_ods_content_xml(columns, data, indices);
    let manifest_xml = build_ods_manifest_xml();

    let mut zip = ZipWriter::new();
    // mimetype MUST be the first entry, stored without compression.
    zip.add_file("mimetype", mimetype);
    zip.add_file("content.xml", content_xml.as_bytes());
    zip.add_file("META-INF/manifest.xml", manifest_xml.as_bytes());
    zip.finish()
}

// ── CSV helpers ────────────────────────────────────────────────────

fn csv_escape(out: &mut String, field: &str) {
    if field.contains(',') || field.contains('"') || field.contains('\n') {
        out.push('"');
        for ch in field.chars() {
            if ch == '"' { out.push('"'); }
            out.push(ch);
        }
        out.push('"');
    } else {
        out.push_str(field);
    }
}

// ── JSON helpers ───────────────────────────────────────────────────

fn json_escape(out: &mut String, s: &str) {
    for ch in s.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c < '\x20' => {
                // Control characters as unicode escapes.
                out.push_str(&format!("\\u{:04x}", c as u32));
            }
            other => out.push(other),
        }
    }
}

// ── ODS XML builders ───────────────────────────────────────────────

fn xml_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            other => out.push(other),
        }
    }
    out
}

fn build_ods_content_xml<T: Clone + PartialEq>(
    columns: &[EqColumnDef<T>],
    data: &[T],
    indices: &[usize],
) -> String {
    let mut xml = String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <office:document-content \
           xmlns:office=\"urn:oasis:names:tc:opendocument:xmlns:office:1.0\" \
           xmlns:text=\"urn:oasis:names:tc:opendocument:xmlns:text:1.0\" \
           xmlns:table=\"urn:oasis:names:tc:opendocument:xmlns:table:1.0\" \
           office:version=\"1.2\">\n\
         <office:body>\n\
         <office:spreadsheet>\n\
         <table:table table:name=\"Export\">\n"
    );

    // Header row
    xml.push_str("<table:table-row>\n");
    for col in columns {
        xml.push_str("<table:table-cell office:value-type=\"string\"><text:p>");
        xml.push_str(&xml_escape(col.header));
        xml.push_str("</text:p></table:table-cell>\n");
    }
    xml.push_str("</table:table-row>\n");

    // Data rows
    for &idx in indices {
        let row = &data[idx];
        xml.push_str("<table:table-row>\n");
        for col in columns {
            let val = (col.value_getter)(row);
            // Try to detect numeric values for proper ODS typing.
            if let Ok(num) = val.trim().parse::<f64>() {
                xml.push_str(&format!(
                    "<table:table-cell office:value-type=\"float\" office:value=\"{}\"><text:p>{}</text:p></table:table-cell>\n",
                    num,
                    xml_escape(&val)
                ));
            } else {
                xml.push_str("<table:table-cell office:value-type=\"string\"><text:p>");
                xml.push_str(&xml_escape(&val));
                xml.push_str("</text:p></table:table-cell>\n");
            }
        }
        xml.push_str("</table:table-row>\n");
    }

    xml.push_str(
        "</table:table>\n\
         </office:spreadsheet>\n\
         </office:body>\n\
         </office:document-content>"
    );

    xml
}

fn build_ods_manifest_xml() -> String {
    String::from(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
         <manifest:manifest xmlns:manifest=\"urn:oasis:names:tc:opendocument:xmlns:manifest:1.0\" \
           manifest:version=\"1.2\">\n\
         <manifest:file-entry manifest:full-path=\"/\" \
           manifest:media-type=\"application/vnd.oasis.opendocument.spreadsheet\"/>\n\
         <manifest:file-entry manifest:full-path=\"content.xml\" \
           manifest:media-type=\"text/xml\"/>\n\
         </manifest:manifest>"
    )
}

// ── Minimal ZIP writer (STORED, no compression) ────────────────────
//
// Produces valid ZIP archives with the STORED method (no compression).
// This keeps eq_ui dependency-free while supporting ODS output.

/// CRC32 lookup table (IEEE/ISO 3309 polynomial 0xEDB88320).
const CRC32_TABLE: [u32; 256] = {
    let mut table = [0u32; 256];
    let mut i = 0u32;
    while i < 256 {
        let mut crc = i;
        let mut j = 0;
        while j < 8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ 0xEDB88320;
            } else {
                crc >>= 1;
            }
            j += 1;
        }
        table[i as usize] = crc;
        i += 1;
    }
    table
};

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        let idx = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ CRC32_TABLE[idx];
    }
    crc ^ 0xFFFF_FFFF
}

struct ZipEntry {
    name: Vec<u8>,
    data: Vec<u8>,
    crc: u32,
    offset: u32,
}

struct ZipWriter {
    entries: Vec<ZipEntry>,
    body: Vec<u8>,
}

impl ZipWriter {
    fn new() -> Self {
        Self {
            entries: Vec::new(),
            body: Vec::new(),
        }
    }

    fn add_file(&mut self, name: &str, data: &[u8]) {
        let crc = crc32(data);
        let offset = self.body.len() as u32;
        let name_bytes = name.as_bytes();
        let data_len = data.len() as u32;

        // Local file header (30 bytes + filename + data)
        self.body.extend_from_slice(&0x04034b50u32.to_le_bytes()); // signature
        self.body.extend_from_slice(&20u16.to_le_bytes());         // version needed
        self.body.extend_from_slice(&0u16.to_le_bytes());          // flags
        self.body.extend_from_slice(&0u16.to_le_bytes());          // compression: STORED
        self.body.extend_from_slice(&0u16.to_le_bytes());          // mod time
        self.body.extend_from_slice(&0u16.to_le_bytes());          // mod date
        self.body.extend_from_slice(&crc.to_le_bytes());           // crc32
        self.body.extend_from_slice(&data_len.to_le_bytes());      // compressed size
        self.body.extend_from_slice(&data_len.to_le_bytes());      // uncompressed size
        self.body.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes()); // name len
        self.body.extend_from_slice(&0u16.to_le_bytes());          // extra field len
        self.body.extend_from_slice(name_bytes);                   // filename
        self.body.extend_from_slice(data);                         // file data

        self.entries.push(ZipEntry {
            name: name_bytes.to_vec(),
            data: data.to_vec(),
            crc,
            offset,
        });
    }

    fn finish(mut self) -> Vec<u8> {
        let central_dir_offset = self.body.len() as u32;

        // Central directory entries
        for entry in &self.entries {
            let data_len = entry.data.len() as u32;

            self.body.extend_from_slice(&0x02014b50u32.to_le_bytes()); // signature
            self.body.extend_from_slice(&20u16.to_le_bytes());         // version made by
            self.body.extend_from_slice(&20u16.to_le_bytes());         // version needed
            self.body.extend_from_slice(&0u16.to_le_bytes());          // flags
            self.body.extend_from_slice(&0u16.to_le_bytes());          // compression
            self.body.extend_from_slice(&0u16.to_le_bytes());          // mod time
            self.body.extend_from_slice(&0u16.to_le_bytes());          // mod date
            self.body.extend_from_slice(&entry.crc.to_le_bytes());     // crc32
            self.body.extend_from_slice(&data_len.to_le_bytes());      // compressed size
            self.body.extend_from_slice(&data_len.to_le_bytes());      // uncompressed size
            self.body.extend_from_slice(&(entry.name.len() as u16).to_le_bytes()); // name len
            self.body.extend_from_slice(&0u16.to_le_bytes());          // extra field len
            self.body.extend_from_slice(&0u16.to_le_bytes());          // comment len
            self.body.extend_from_slice(&0u16.to_le_bytes());          // disk number
            self.body.extend_from_slice(&0u16.to_le_bytes());          // internal attrs
            self.body.extend_from_slice(&0u32.to_le_bytes());          // external attrs
            self.body.extend_from_slice(&entry.offset.to_le_bytes());  // local header offset
            self.body.extend_from_slice(&entry.name);                  // filename
        }

        let central_dir_size = self.body.len() as u32 - central_dir_offset;
        let entry_count = self.entries.len() as u16;

        // End of central directory record
        self.body.extend_from_slice(&0x06054b50u32.to_le_bytes()); // signature
        self.body.extend_from_slice(&0u16.to_le_bytes());          // disk number
        self.body.extend_from_slice(&0u16.to_le_bytes());          // disk with central dir
        self.body.extend_from_slice(&entry_count.to_le_bytes());   // entries on this disk
        self.body.extend_from_slice(&entry_count.to_le_bytes());   // total entries
        self.body.extend_from_slice(&central_dir_size.to_le_bytes()); // central dir size
        self.body.extend_from_slice(&central_dir_offset.to_le_bytes()); // central dir offset
        self.body.extend_from_slice(&0u16.to_le_bytes());          // comment length

        self.body
    }
}
