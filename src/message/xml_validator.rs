use crate::lib::parser::XmlSchemaParser;

pub fn xml_validator(xml: &str) -> bool {
    let xsd_data = r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:20022:tech:xsd:pain.001.001.03">
    <CstmrCdtTrfInitn>
        <GrpHdr>
            <MsgId>1234567890</MsgId>
            <CreDtTm>2024-10-11T10:00:00</CreDtTm>
            <NbOfTxs>1</NbOfTxs>
            <CtrlSum>1500.00</CtrlSum>
            <InitgPty>
                <Nm>John Doe</Nm>
                <Id>
                    <OrgId>
                        <Othr>
                            <Id>123456789</Id>
                        </Othr>
                    </OrgId>
                </Id>
            </InitgPty>
        </GrpHdr>
        <PmtInf>
            <PmtInfId>987654321</PmtInfId>
            <PmtMtd>TRF</PmtMtd>
            <BtchBookg>false</BtchBookg>
            <NbOfTxs>1</NbOfTxs>
            <CtrlSum>1500.00</CtrlSum>
            <PmtTpInf>
                <SvcLvl>
                    <Cd>SEPA</Cd>
                </SvcLvl>
                <LclInstrm>
                    <Cd>CORE</Cd>
                </LclInstrm>
                <SeqTp>FRST</SeqTp>
            </PmtTpInf>
            <ReqdExctnDt>2024-10-11</ReqdExctnDt>
            <Dbtr>
                <Nm>John Doe</Nm>
                <Id>
                    <OrgId>
                        <Othr>
                            <Id>123456789</Id>
                        </Othr>
                    </OrgId>
                </Id>
                <CtryOfRes>US</CtryOfRes>
            </Dbtr>
            <DbtrAcct>
                <Id>
                    <IBAN>US1234567890123456789012345</IBAN>
                </Id>
            </DbtrAcct>
            <CdtTrfTxInf>
                <PmtId>
                    <EndToEndId>ABC123</EndToEndId>
                </PmtId>
                <Amt>
                    <InstdAmt Ccy="USD">1500.00</InstdAmt>
                </Amt>
                <Cdtr>
                    <Nm>Jane Smith</Nm>
                    <Id>
                        <OrgId>
                            <Othr>
                                <Id>987654321</Id>
                            </Othr>
                        </OrgId>
                    </Id>
                    <CtryOfRes>GB</CtryOfRes>
                </Cdtr>
                <CdtrAcct>
                    <Id>
                        <IBAN>GB9876543210123456789012345</IBAN>
                    </Id>
                </CdtrAcct>
                <RmtInf>
                    <Ustrd>Invoice payment</Ustrd>
                </RmtInf>
            </CdtTrfTxInf>
        </PmtInf>
    </CstmrCdtTrfInitn>
</Document>"#;

    let mut parser = XmlSchemaParser::new(xsd_data);

    let _ = parser.parse();

    let result = parser.validate(xml);

    result
}
