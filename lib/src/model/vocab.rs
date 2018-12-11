//! Provides ready to use `NamedNode`s for basic RDF vocabularies

pub mod rdf {
    //! [RDF 1.1](https://www.w3.org/TR/rdf11-concepts/) vocabulary
    use crate::model::named_node::NamedNode;
    use lazy_static::lazy_static;
    use std::str::FromStr;

    lazy_static! {
        /// The class of containers of alternatives.
        pub static ref ALT: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt").unwrap();
        /// The class of unordered containers.
        pub static ref BAG: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag").unwrap();
        /// The first item in the subject RDF list.
        pub static ref FIRST: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#first").unwrap();
        /// The class of HTML literal values.
        pub static ref HTML: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#HTML").unwrap();
        /// The class of language-tagged string literal values.
        pub static ref LANG_STRING: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString").unwrap();
        /// The class of RDF Lists.
        pub static ref LIST: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#List").unwrap();
        pub static ref NIL: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#nil").unwrap();
        /// The object of the subject RDF statement.
        pub static ref OBJECT: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#object").unwrap();
        /// The predicate of the subject RDF statement.
        pub static ref PREDICATE: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate").unwrap();
        /// The class of RDF properties.
        pub static ref PROPERTY: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Property").unwrap();
        /// The rest of the subject RDF list after the first item.
        pub static ref REST: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#rest").unwrap();
        /// The class of ordered containers.
        pub static ref SEQ: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq").unwrap();
        /// The class of RDF statements.
        pub static ref STATEMENT: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement").unwrap();
        /// The subject of the subject RDF statement.
        pub static ref SUBJECT: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject").unwrap();
        /// The subject is an instance of a class.
        pub static ref TYPE: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").unwrap();
        /// Idiomatic property used for structured values.
        pub static ref VALUE: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#value").unwrap();
        /// The class of XML literal values.
        pub static ref XML_LITERAL: NamedNode =
            NamedNode::from_str("http://www.w3.org/1999/02/22-rdf-syntax-ns#XMLLiteral").unwrap();
    }
}

pub mod rdfs {
    //! [RDFS](https://www.w3.org/TR/rdf-schema/) vocabulary
    use crate::model::named_node::NamedNode;
    use lazy_static::lazy_static;
    use std::str::FromStr;

    lazy_static! {
        /// The class of classes.
        pub static ref CLASS: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#Class").unwrap();
        /// A description of the subject resource.
        pub static ref COMMENT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#comment").unwrap();
        /// The class of RDF containers.
        pub static ref CONTAINER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#Container").unwrap();
        /// The class of container membership properties, rdf:_1, rdf:_2, ..., all of which are sub-properties of 'member'.
        pub static ref CONTAINER_MEMBERSHIP_PROPERTY: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty").unwrap();
        /// The class of RDF datatypes.
        pub static ref DATATYPE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#Datatype").unwrap();
        /// A domain of the subject property.
        pub static ref DOMAIN: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#domain").unwrap();
        /// The definition of the subject resource.
        pub static ref IS_DEFINED_BY: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#isDefinedBy").unwrap();
        /// A human-readable name for the subject.
        pub static ref LABEL: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#label").unwrap();
        /// The class of literal values, e.g. textual strings and integers.
        pub static ref LITERAL: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#Literal").unwrap();
        /// A member of the subject resource.
        pub static ref MEMBER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#member").unwrap();
        /// A range of the subject property.
        pub static ref RANGE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#range").unwrap();
        /// The class resource, everything.
        pub static ref RESOURCE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#Resource").unwrap();
        /// Further information about the subject resource.
        pub static ref SEE_ALSO: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#seeAlso").unwrap();
        /// The subject is a subclass of a class.
        pub static ref SUB_CLASS_OF: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#subClassOf").unwrap();
        /// The subject is a subproperty of a property.
        pub static ref SUB_PROPERTY_OF: NamedNode =
            NamedNode::from_str("http://www.w3.org/2000/01/rdf-schema#subPropertyOf").unwrap();
    }
}

pub mod xsd {
    //! `NamedNode`s for [RDF compatible XSD datatypes](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-compatible-xsd-types)
    use crate::model::named_node::NamedNode;
    use lazy_static::lazy_static;
    use std::str::FromStr;

    lazy_static! {
        /// true, false
        pub static ref BOOLEAN: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#boolean").unwrap();
        /// 128…+127 (8 bit)
        pub static ref BYTE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#byte").unwrap();
        /// Dates (yyyy-mm-dd) with or without timezone
        pub static ref DATE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#date").unwrap();
        /// Duration of time (days, hours, minutes, seconds only)
        pub static ref DAY_TIME_DURATION: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#dayTimeDuration").unwrap();
        /// Date and time with or without timezone
        pub static ref DATE_TIME: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#dateTime").unwrap();
        /// Date and time with required timezone
        pub static ref DATE_TIME_STAMP: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#dateTimeStamp").unwrap();
        /// Arbitrary-precision decimal numbers
        pub static ref DECIMAL: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#decimal").unwrap();
        /// 64-bit floating point numbers incl. ±Inf, ±0, NaN
        pub static ref DOUBLE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#double").unwrap();
        /// Duration of time
        pub static ref DURATION: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#duration").unwrap();
        /// 32-bit floating point numbers incl. ±Inf, ±0, NaN
        pub static ref FLOAT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#float").unwrap();
        /// Gregorian calendar day of the month
        pub static ref G_DAY: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gDay").unwrap();
        /// Gregorian calendar month
        pub static ref G_MONTH: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gMonth").unwrap();
        /// Gregorian calendar month and day
        pub static ref G_MONTH_DAY: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gMonthDay").unwrap();
        /// Gregorian calendar year
        pub static ref G_YEAR: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gYear").unwrap();
        /// Gregorian calendar year and month
        pub static ref G_YEAR_MONTH: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#gYearMonth").unwrap();
        /// -2147483648…+2147483647 (32 bit)
        pub static ref INT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#int").unwrap();
        /// Arbitrary-size integer numbers
        pub static ref INTEGER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#integer").unwrap();
        /// -9223372036854775808…+9223372036854775807 (64 bit)
        pub static ref LONG: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#long").unwrap();
        /// Integer numbers <0
        pub static ref NEGATIVE_INTEGER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#negativeInteger").unwrap();
        /// Integer numbers ≥0
        pub static ref NON_NEGATIVE_INTEGER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#nonNegativeInteger").unwrap();
        /// Integer numbers ≤0
        pub static ref NON_POSITIVE_INTEGER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#nonPositiveInteger").unwrap();
        /// Integer numbers >0
        pub static ref POSITIVE_INTEGER: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#positiveInteger").unwrap();
        /// Times (hh:mm:ss.sss…) with or without timezone
        pub static ref TIME: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#time").unwrap();
        /// -32768…+32767 (16 bit)
        pub static ref SHORT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#short").unwrap();
        /// Character strings (but not all Unicode character strings)
        pub static ref STRING: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#string").unwrap();
        /// 0…255 (8 bit)
        pub static ref UNSIGNED_BYTE: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#unsignedByte").unwrap();
        /// 0…4294967295 (32 bit)
        pub static ref UNSIGNED_INT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#unsignedInt").unwrap();
        /// 0…18446744073709551615 (64 bit)
        pub static ref UNSIGNED_LONG: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#unsignedLong").unwrap();
        /// 0…65535 (16 bit)
        pub static ref UNSIGNED_SHORT: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#unsignedShort").unwrap();
        /// Duration of time (months and years only)
        pub static ref YEAR_MONTH_DURATION: NamedNode =
            NamedNode::from_str("http://www.w3.org/2001/XMLSchema#yearMonthDuration").unwrap();
    }
}
