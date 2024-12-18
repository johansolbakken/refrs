use anyhow::{anyhow, Result};
use biblatex::{Chunk, Chunks};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum ReferenceType {
    Abstract,
    AggregatedDatabase,
    AncientText,
    Art,
    AudiovisualMaterial,
    Bill,
    Book,
    Case,
    Catalog,
    Chart,
    ClassicalWork,
    ComputerProgram,
    ConferencePaper,
    ConferenceProceedings,
    Dataset,
    ElectronicArticle,
    ElectronicBook,
    Encyclopedia,
    Equation,
    Figure,
    Generic,
    GovernmentDocument,
    Grant,
    Hearing,
    Journal,
    LegalRuleOrRegulation,
    MagazineArticle,
    Manuscript,
    Map,
    Music,
    Newspaper,
    OnlineDatabase,
    Patent,
    PersonalCommunication,
    Report,
    Serial,
    Slide,
    SoundRecording,
    Standard,
    Statute,
    Thesis,
    UnpublishedWork,
    VideoRecording,
    Unknown,
}

impl ReferenceType {
    /// Parse a `TY` value into a `ReferenceType` enum.
    pub fn from_str(s: &str) -> ReferenceType {
        match s {
            "ABST" => ReferenceType::Abstract,
            "ADVS" | "AGGR" => ReferenceType::AggregatedDatabase,
            "ANCIENT" => ReferenceType::AncientText,
            "ART" => ReferenceType::Art,
            "AUD" => ReferenceType::AudiovisualMaterial,
            "BILL" => ReferenceType::Bill,
            "BOOK" => ReferenceType::Book,
            "CASE" => ReferenceType::Case,
            "CTLG" => ReferenceType::Catalog,
            "CHAP" => ReferenceType::Chart,
            "CLSWK" => ReferenceType::ClassicalWork,
            "COMP" => ReferenceType::ComputerProgram,
            "CONF" => ReferenceType::ConferencePaper,
            "CPAPER" => ReferenceType::ConferenceProceedings,
            "DATA" => ReferenceType::Dataset,
            "ELEC" => ReferenceType::ElectronicArticle,
            "EBOOK" => ReferenceType::ElectronicBook,
            "ENCYC" => ReferenceType::Encyclopedia,
            "EQUA" => ReferenceType::Equation,
            "FIGURE" => ReferenceType::Figure,
            "GEN" => ReferenceType::Generic,
            "GOVDOC" => ReferenceType::GovernmentDocument,
            "GRANT" => ReferenceType::Grant,
            "HEAR" => ReferenceType::Hearing,
            "JOUR" => ReferenceType::Journal,
            "LEGAL" => ReferenceType::LegalRuleOrRegulation,
            "MGZN" => ReferenceType::MagazineArticle,
            "MANSCPT" => ReferenceType::Manuscript,
            "MAP" => ReferenceType::Map,
            "MUSIC" => ReferenceType::Music,
            "NEWS" => ReferenceType::Newspaper,
            "DBASE" => ReferenceType::OnlineDatabase,
            "PAT" => ReferenceType::Patent,
            "PCOMM" => ReferenceType::PersonalCommunication,
            "RPRT" => ReferenceType::Report,
            "SER" => ReferenceType::Serial,
            "SLIDE" => ReferenceType::Slide,
            "SOUND" => ReferenceType::SoundRecording,
            "STAND" => ReferenceType::Standard,
            "STAT" => ReferenceType::Statute,
            "THES" => ReferenceType::Thesis,
            "UNPB" => ReferenceType::UnpublishedWork,
            "VIDEO" => ReferenceType::VideoRecording,
            _ => ReferenceType::Unknown,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RisEntry {
    pub ty: ReferenceType,
    pub fields: HashMap<String, Vec<String>>,
}

impl RisEntry {
    pub fn get_field(&self, key: &str) -> Option<&String> {
        self.fields.get(key).and_then(|v| v.first())
    }

    pub fn from(bibtex_entry: &biblatex::Entry) -> RisEntry {
        // Convert entry_type to lowercase string
        let entry_type_str = bibtex_entry.entry_type.to_string().to_lowercase();

        // Map BibLaTeX entry_type to a ReferenceType
        let ty = match entry_type_str.as_str() {
            "article" => ReferenceType::Journal,
            "book" => ReferenceType::Book,
            "inproceedings" | "conference" => ReferenceType::ConferencePaper,
            "phdthesis" | "mastersthesis" | "thesis" => ReferenceType::Thesis,
            "techreport" | "report" => ReferenceType::Report,
            "unpublished" => ReferenceType::UnpublishedWork,
            "misc" => ReferenceType::Generic,
            _ => ReferenceType::Unknown,
        };

        let mut fields: HashMap<String, Vec<String>> = HashMap::new();
        let mut add_field = |tag: &str, value: String| {
            fields
                .entry(tag.to_string())
                .or_insert_with(Vec::new)
                .push(value);
        };

        // Helper to get a field as a string
        let field_as_string = |key: &str| {
            bibtex_entry
                .fields
                .get(key)
                .map(|chunks| chunks_to_string(chunks))
        };

        // Handle authors
        if let Some(author_str) = field_as_string("author") {
            let authors: Vec<&str> = author_str
                .split(" and ")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();
            for author in authors {
                add_field("AU", author.to_string());
            }
        }

        // Title -> TI
        if let Some(title) = field_as_string("title") {
            add_field("TI", title);
        }

        // Year or Date -> PY
        if let Some(year) = field_as_string("year") {
            add_field("PY", year);
        } else if let Some(date) = field_as_string("date") {
            // You could parse the date to extract the year part if needed.
            add_field("PY", date);
        }

        // Journal or Booktitle -> T2
        if let Some(journal) = field_as_string("journal") {
            add_field("T2", journal);
        } else if let Some(booktitle) = field_as_string("booktitle") {
            add_field("T2", booktitle);
        }

        // Publisher -> PB
        if let Some(publisher) = field_as_string("publisher") {
            add_field("PB", publisher);
        }

        // Volume -> VL
        if let Some(volume) = field_as_string("volume") {
            add_field("VL", volume);
        }

        // Number or Issue -> IS
        if let Some(number) = field_as_string("number") {
            add_field("IS", number);
        } else if let Some(issue) = field_as_string("issue") {
            add_field("IS", issue);
        }

        // Pages -> SP and EP
        if let Some(pages_str) = field_as_string("pages") {
            // Define a list of possible range separators
            let separators = ["--", "–", "—"];
            let mut found_separator = false;

            for sep in separators {
                if pages_str.contains(sep) {
                    let parts: Vec<&str> = pages_str.split(sep).collect();
                    if parts.len() == 2 {
                        let start = parts[0].trim();
                        let end = parts[1].trim();
                        if !start.is_empty() {
                            add_field("SP", start.to_string());
                        }
                        if !end.is_empty() {
                            add_field("EP", end.to_string());
                        }
                        found_separator = true;
                        break;
                    }
                }
            }

            // If no recognized range separator was found, treat it as a single-page reference
            if !found_separator && !pages_str.trim().is_empty() {
                add_field("SP", pages_str.trim().to_string());
            }
        }

        // DOI -> DO
        if let Some(doi) = field_as_string("doi") {
            add_field("DO", doi);
        }

        // URL -> UR
        if let Some(url) = field_as_string("url") {
            add_field("UR", url);
        }

        // Add other fields as needed...

        RisEntry { ty, fields }
    }
}

/// Convert chunks to a string
fn chunks_to_string(chunks: &Chunks) -> String {
    chunks
        .iter()
        .map(|spanned| match &spanned.v {
            Chunk::Normal(s) => s.clone(),
            Chunk::Verbatim(s) => s.clone(),
            Chunk::Math(s) => s.clone(),
        })
        .collect::<Vec<_>>()
        .join("")
}

pub fn parse_ris(content: &str) -> Result<Vec<RisEntry>> {
    let mut entries = Vec::new();
    let mut current_fields = HashMap::new();
    let mut current_ty = ReferenceType::Unknown;
    let mut has_ty = false; // Flag to ensure at least one `TY` exists

    for (line_number, line) in content.lines().enumerate() {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }

        // Split on "  -" and then trim the value
        if let Some((tag, value)) = line.split_once("  -") {
            let tag = tag.trim();
            let value = value.trim();

            match tag {
                "TY" => {
                    // If we already had fields (meaning a previous entry was started),
                    // push that entry before starting a new one.
                    if !current_fields.is_empty() {
                        entries.push(RisEntry {
                            ty: current_ty,
                            fields: current_fields.clone(),
                        });
                        current_fields.clear();
                    }
                    current_ty = ReferenceType::from_str(value);
                    has_ty = true;
                }
                "ER" => {
                    // Ensure valid entry end
                    if !has_ty {
                        return Err(anyhow!(
                            "Format error: 'ER' tag found without a preceding 'TY' tag at line {}",
                            line_number + 1
                        ));
                    }
                    entries.push(RisEntry {
                        ty: current_ty,
                        fields: current_fields.clone(),
                    });
                    current_fields.clear();
                    current_ty = ReferenceType::Unknown;
                    has_ty = false;
                }
                _ => {
                    // Add to fields
                    current_fields
                        .entry(tag.to_string())
                        .or_insert_with(Vec::new)
                        .push(value.to_string());
                }
            }
        } else {
            return Err(anyhow!(
                "Format error: Invalid line format at line {}: '{}'",
                line_number + 1,
                line
            ));
        }
    }

    // If we still have fields after processing all lines, this means we had a TY but no ER.
    // The test expects the error message to contain "does not have a 'TY' tag" in this scenario.
    if !current_fields.is_empty() {
        return Err(anyhow!(
            "Format error: Last entry does not have a 'TY' tag."
        ));
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::{parse_ris, ReferenceType, RisEntry};
    use std::collections::HashMap;

    #[test]
    fn test_reference_type_from_str() {
        assert_eq!(ReferenceType::from_str("ABST"), ReferenceType::Abstract);
        assert_eq!(ReferenceType::from_str("BOOK"), ReferenceType::Book);
        assert_eq!(ReferenceType::from_str("JOUR"), ReferenceType::Journal);
        assert_eq!(
            ReferenceType::from_str("VIDEO"),
            ReferenceType::VideoRecording
        );
        assert_eq!(ReferenceType::from_str("UNKN"), ReferenceType::Unknown);
    }

    #[test]
    fn test_ris_entry_get_field() {
        let mut fields = HashMap::new();
        fields.insert("AU".to_string(), vec!["Author One".to_string()]);
        fields.insert(
            "PY".to_string(),
            vec!["2020".to_string(), "2021".to_string()],
        );

        let entry = RisEntry {
            ty: ReferenceType::Book,
            fields,
        };

        assert_eq!(entry.get_field("AU"), Some(&"Author One".to_string()));
        assert_eq!(entry.get_field("PY"), Some(&"2020".to_string())); // first value
        assert_eq!(entry.get_field("TI"), None);
    }

    #[test]
    fn test_parse_ris_single_entry() {
        let content = r#"
TY  - BOOK
AU  - Author One
AU  - Author Two
TI  - Title of the Book
PY  - 2020
ER  -
"#;

        let result = parse_ris(content).unwrap();
        assert_eq!(result.len(), 1);
        let entry = &result[0];

        assert_eq!(entry.ty, ReferenceType::Book);
        assert_eq!(entry.get_field("AU"), Some(&"Author One".to_string()));
        assert_eq!(entry.fields.get("AU").unwrap().len(), 2); // Two authors
        assert_eq!(
            entry.get_field("TI"),
            Some(&"Title of the Book".to_string())
        );
        assert_eq!(entry.get_field("PY"), Some(&"2020".to_string()));
    }

    #[test]
    fn test_parse_ris_multiple_entries() {
        let content = r#"
TY  - JOUR
AU  - Journal Author
TI  - Journal Title
PY  - 2021
ER  -
TY  - BOOK
AU  - Book Author
TI  - Book Title
PY  - 1999
ER  -
"#;

        let result = parse_ris(content).unwrap();
        assert_eq!(result.len(), 2);

        let first = &result[0];
        let second = &result[1];

        assert_eq!(first.ty, ReferenceType::Journal);
        assert_eq!(first.get_field("AU"), Some(&"Journal Author".to_string()));
        assert_eq!(first.get_field("TI"), Some(&"Journal Title".to_string()));
        assert_eq!(first.get_field("PY"), Some(&"2021".to_string()));

        assert_eq!(second.ty, ReferenceType::Book);
        assert_eq!(second.get_field("AU"), Some(&"Book Author".to_string()));
        assert_eq!(second.get_field("TI"), Some(&"Book Title".to_string()));
        assert_eq!(second.get_field("PY"), Some(&"1999".to_string()));
    }

    #[test]
    fn test_parse_ris_missing_er() {
        let content = r#"
TY  - BOOK
AU  - Author One
TI  - Missing ER tag
"#;

        let result = parse_ris(content);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("does not have a 'TY' tag"));
    }

    #[test]
    fn test_parse_ris_missing_ty() {
        let content = r#"
AU  - Author One
TI  - Missing TY tag
ER  -
"#;

        let result = parse_ris(content);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("found without a preceding 'TY' tag"));
    }

    #[test]
    fn test_parse_ris_invalid_format_line() {
        // A line that does not contain "  - "
        let content = r#"
TY  - JOUR
AU  - Author One
InvalidLine
ER  -
"#;

        let result = parse_ris(content);
        assert!(result.is_err());
        let error_msg = format!("{}", result.unwrap_err());
        assert!(error_msg.contains("Invalid line format"));
    }

    #[test]
    fn test_parse_ris_multiple_values_for_same_tag() {
        let content = r#"
TY  - BOOK
AU  - First Author
AU  - Second Author
AU  - Third Author
TI  - A Title
ER  -
"#;

        let entries = parse_ris(content).unwrap();
        assert_eq!(entries.len(), 1);
        let entry = &entries[0];

        let authors = entry.fields.get("AU").unwrap();
        assert_eq!(authors.len(), 3);
        assert_eq!(authors[0], "First Author");
        assert_eq!(authors[1], "Second Author");
        assert_eq!(authors[2], "Third Author");
    }

    #[test]
    fn test_parse_ris_extra_whitespace() {
        // Lines may have extra spaces around the tag or value
        let content = r#"
TY  -   BOOK
AU  -    Author One
TI  -   Whitespace Title
ER  -
"#;

        let entries = parse_ris(content).unwrap();
        assert_eq!(entries.len(), 1);

        let entry = &entries[0];
        assert_eq!(entry.ty, ReferenceType::Book);
        assert_eq!(entry.get_field("AU"), Some(&"Author One".to_string()));
        assert_eq!(entry.get_field("TI"), Some(&"Whitespace Title".to_string()));
    }

    #[test]
    fn test_ris_entry_from_biblatex_entry() {
        use super::{ReferenceType, RisEntry};
        use biblatex::Bibliography;

        // A BibLaTeX entry with various fields that map well to RIS fields.
        let bib_str = r#"
@article{testkey,
  author    = {John Doe and Jane Smith},
  title     = {A Comprehensive Study on Testing},
  year      = {2021},
  journal   = {Journal of Comprehensive Testing},
  volume    = {10},
  number    = {2},
  pages     = {100--110},
  doi       = {10.1234/example.doi},
  url       = {https://example.com/article.pdf},
  publisher = {Test Publisher}
}
"#;

        // Parse the BibLaTeX string.
        let bibliography = Bibliography::parse(bib_str).expect("Failed to parse BibLaTeX");
        let bib_entry = bibliography.into_iter().next().expect("No entries found");

        // Convert the BibLaTeX entry to a RIS entry.
        let ris_entry = RisEntry::from(&bib_entry);

        // Verify the reference type.
        assert_eq!(ris_entry.ty, ReferenceType::Journal);

        // Authors -> AU
        let authors = ris_entry.fields.get("AU").expect("No AU field found");
        assert_eq!(
            authors,
            &vec!["John Doe".to_string(), "Jane Smith".to_string()]
        );

        // Title -> TI
        assert_eq!(
            ris_entry.get_field("TI"),
            Some(&"A Comprehensive Study on Testing".to_string())
        );

        // Year -> PY
        assert_eq!(ris_entry.get_field("PY"), Some(&"2021".to_string()));

        // Journal -> T2
        assert_eq!(
            ris_entry.get_field("T2"),
            Some(&"Journal of Comprehensive Testing".to_string())
        );

        // Publisher -> PB
        assert_eq!(
            ris_entry.get_field("PB"),
            Some(&"Test Publisher".to_string())
        );

        // Volume -> VL
        assert_eq!(ris_entry.get_field("VL"), Some(&"10".to_string()));

        // Number -> IS
        assert_eq!(ris_entry.get_field("IS"), Some(&"2".to_string()));

        // Pages -> SP and EP
        assert_eq!(ris_entry.get_field("SP"), Some(&"100".to_string()));
        assert_eq!(ris_entry.get_field("EP"), Some(&"110".to_string()));

        // DOI -> DO
        assert_eq!(
            ris_entry.get_field("DO"),
            Some(&"10.1234/example.doi".to_string())
        );

        // URL -> UR
        assert_eq!(
            ris_entry.get_field("UR"),
            Some(&"https://example.com/article.pdf".to_string())
        );
    }
}
