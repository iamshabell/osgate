use serde::Deserialize;
use serde_xml_rs::from_str;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

#[derive(Debug, Deserialize, PartialEq)]
struct GroupHeader {
    #[serde(rename = "MsgId")]
    msg_id: Option<String>,
    #[serde(rename = "CreDtTm")]
    creation_date_time: Option<String>,
    #[serde(rename = "NbOfTxs")]
    number_of_transactions: Option<String>,
    #[serde(rename = "InitgPty")]
    initiating_party: Option<InitiatingParty>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct InitiatingParty {
    #[serde(rename = "Nm")]
    name: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PaymentInfo {
    #[serde(rename = "PmtInfId")]
    payment_info_id: Option<String>,

    #[serde(rename = "CdtTrfTxInf")]
    credit_transfer_transaction_info: Vec<CreditTransferTransactionInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct CreditTransferTransactionInfo {
    #[serde(rename = "PmtId")]
    payment_id: PaymentId,

    #[serde(rename = "Amt")]
    amount: Amount,

    #[serde(rename = "Cdtr")]
    creditor: Party,

    #[serde(rename = "CdtrAcct")]
    creditor_account: Account,

    //#[serde(rename = "CdtrAgt")]
    //creditor_agent: Agent,
    #[serde(rename = "RmtInf")]
    remittance_information: RemittanceInformation,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PaymentId {
    #[serde(rename = "InstrId")]
    instruction_id: Option<String>,
    #[serde(rename = "EndToEndId")]
    end_to_end_id: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Amount {
    #[serde(rename = "InstdAmt")]
    instructed_amount: InstdAmt,
}

#[derive(Debug, Deserialize, PartialEq)]
struct InstdAmt {
    #[serde(rename = "$value")]
    amount: String,

    #[serde(rename = "Ccy")]
    currency: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Party {
    #[serde(rename = "Nm")]
    name: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Account {
    #[serde(rename = "Id")]
    id: AccountId,
}

#[derive(Debug, Deserialize, PartialEq)]
struct AccountId {
    #[serde(rename = "IBAN")]
    iban: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Agent {
    #[serde(rename = "FinInstnId")]
    financial_institution_id: FinancialInstitutionId,
}

#[derive(Debug, Deserialize, PartialEq)]
struct FinancialInstitutionId {
    #[serde(rename = "BIC")]
    bic: String,
}

#[derive(Debug, Deserialize, PartialEq)]
struct RemittanceInformation {
    #[serde(rename = "Ustrd")]
    unstructured: Option<String>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct PaymentInformation {
    #[serde(rename = "PmtInfId")]
    payment_info_id: Option<String>,

    #[serde(rename = "CdtTrfTxInf")]
    credit_transfer_transaction_info: Vec<CreditTransferTransactionInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct CustomerCreditTransferInitiation {
    #[serde(rename = "GrpHdr")]
    group_header: GroupHeader,

    #[serde(rename = "PmtInf")]
    payment_info: Vec<PaymentInfo>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Pain001 {
    #[serde(rename = "CstmrCdtTrfInitn")]
    customer_credit_transfer_initiation: CustomerCreditTransferInitiation,
}

pub struct XmlSchemaParser {
    schema: String,
    current_schema: Pain001,
}

impl XmlSchemaParser {
    pub fn new(schema: &str) -> Self {
        XmlSchemaParser {
            schema: schema.to_string(),
            current_schema: Pain001 {
                customer_credit_transfer_initiation: CustomerCreditTransferInitiation {
                    group_header: GroupHeader {
                        msg_id: None,
                        creation_date_time: None,
                        number_of_transactions: None,
                        initiating_party: None,
                    },
                    payment_info: vec![],
                },
            },
        }
    }

    pub fn parse(&mut self) -> Result<(), String> {
        let parsed_schema = match self.to_type(&self.schema) {
            Ok(parsed_schema) => parsed_schema,
            Err(_) => return Err("Failed to parse schema".to_string()),
        };

        self.current_schema = parsed_schema;
        Ok(())
    }

    pub fn validate(&self, xml: &str) -> bool {
        let pain001 = match self.to_type(xml) {
            Ok(pain001) => pain001,
            Err(_) => return false,
        };

        self.match_schema(pain001)
    }

    fn match_schema(&self, pain001: Pain001) -> bool {
        if self.schema.is_empty() {
            return false;
        }

        if pain001 == self.current_schema {
            return true;
        }

        false
    }

    fn to_type(&self, xml: &str) -> Result<Pain001, Box<dyn Error>> {
        let pain001: Pain001 = from_str(xml)?;

        Ok(pain001)
    }
}
