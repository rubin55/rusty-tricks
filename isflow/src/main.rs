use indexmap::IndexMap;
use itertools::Itertools;

type Certificate = String;
type DocType = String;
type NameSpace = String;
type DataElementIdentifier = String;
type DataElementValue = String;
type ProposedAttributes = IndexMap<DocType, ProposedDocumentAttributes>;

pub struct ProposedDocumentAttributes {
    pub issuer: Certificate,
    pub attributes: IndexMap<NameSpace, Vec<Entry>>,
}

pub struct Entry {
    pub name: DataElementIdentifier,
    pub value: DataElementValue,
}

const PID_DOCTYPE: &str = "cuckles";

pub(crate) fn is_login_flow(proposed_attributes: &ProposedAttributes) -> bool {
    if let Ok((doc_type, doc_attributes)) = proposed_attributes.iter().exactly_one() {
        if doc_type == PID_DOCTYPE {
            if let Ok((namespace, entries)) = doc_attributes.attributes.iter().exactly_one() {
                if namespace == PID_DOCTYPE {
                    if let Ok(entry) = entries.iter().exactly_one() {
                        return entry.name == "foo";
                    }
                }
            }
        }
    }
    false
}

fn main() {
    // First example entry.
    let entry1 = Entry {
        name: "foo".to_string(),
        value: "some-foo-value".to_string(),
    };

    // Second example entry.
    let entry2 = Entry {
        name: "bar".to_string(),
        value: "some-bar-value".to_string(),
    };

    // Insert two namespaces, each with one entry in their attribute maps.
    let mut attributes = IndexMap::new();
    attributes.insert("cuckles".to_string(), vec![entry1]);
    attributes.insert("another".to_string(), vec![entry2]);

    // Create a proposed document attributes structure with above attributes.
    let proposed_document_attributes = ProposedDocumentAttributes {
        issuer: "some-issuer".to_string(),
        attributes,
    };

    // Create a proposed attributes indexmap with one entry containing the above.
    let mut some_proposed_attributes = ProposedAttributes::new();
    some_proposed_attributes.insert("cuckles".to_string(), proposed_document_attributes);

    // Check the is_login_flow function.
    let result = is_login_flow(&some_proposed_attributes);
    println!("Is login flow: {}", result);
}
