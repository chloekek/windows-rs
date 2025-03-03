use windows::{core::*, Win32::Data::Xml::XmlLite::*, Win32::System::Com::StructuredStorage::*, Win32::System::Com::*};

#[test]
fn test() -> Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(None, true)?;

        let mut writer: Option<IXmlWriter> = None;
        CreateXmlWriter(&IXmlWriter::IID, &mut writer as *mut _ as _, None)?;
        let writer = writer.unwrap();
        writer.SetOutput(&stream)?;

        writer.WriteStartDocument(XmlStandalone_Omit)?;
        writer.WriteStartElement(None, w!("html"), None)?;
        writer.WriteElementString(None, w!("head"), None, w!("The quick brown fox jumps over the lazy dog"))?;
        writer.WriteStartElement(None, w!("body"), None)?;
        writer.WriteChars(None)?;
        writer.WriteChars(Some(&[0x52, 0x75, 0x73, 0x74]))?;
        writer.WriteEndDocument()?;
        writer.Flush()?;

        let mut pos = 0;
        stream.Seek(0, STREAM_SEEK_SET, Some(&mut pos))?;
        assert_eq!(pos, 0);

        let mut reader: Option<IXmlReader> = None;
        CreateXmlReader(&IXmlReader::IID, &mut reader as *mut _ as _, None)?;
        let reader = reader.unwrap();
        reader.SetInput(&stream)?;

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_XmlDeclaration);

        let mut name = PCWSTR::null();
        let mut name_len = 0;

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, Some(&mut name_len))?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "html");

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, Some(&mut name_len))?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "head");

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Text);

        let mut message = Vec::new();
        let mut chunk: [u16; 10] = std::mem::zeroed();
        let mut chars_read = 0;
        let mut read_count = 0;

        while reader.ReadValueChunk(&mut chunk, &mut chars_read).is_ok() && chars_read > 0 {
            message.extend_from_slice(&chunk[0..chars_read as _]);
            read_count += 1;
        }

        assert_eq!(read_count, 5);
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(message.as_ptr(), message.len())), "The quick brown fox jumps over the lazy dog");

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_EndElement);

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Text);

        reader.ReadValueChunk(&mut chunk, &mut chars_read).ok()?;
        assert_eq!(chars_read, 4);
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(chunk.as_ptr(), chars_read as _)), "Rust");

        Ok(())
    }
}

#[test]
fn lite() -> Result<()> {
    unsafe {
        let stream = CreateStreamOnHGlobal(None, true)?;

        let mut writer: Option<IXmlWriterLite> = None;
        CreateXmlWriter(&IXmlWriterLite::IID, &mut writer as *mut _ as _, None)?;
        let writer = writer.unwrap();
        writer.SetOutput(&stream)?;

        writer.WriteStartElement(HSTRING::from("html").as_wide())?;
        writer.WriteAttributeString(HSTRING::from("no-value").as_wide(), None)?;
        writer.WriteAttributeString(HSTRING::from("with-value").as_wide(), Some(HSTRING::from("value").as_wide()))?;
        writer.WriteEndElement(HSTRING::from("html").as_wide())?;
        writer.Flush()?;

        let mut pos = 0;
        stream.Seek(0, STREAM_SEEK_SET, Some(&mut pos))?;
        assert_eq!(pos, 0);

        let mut reader: Option<IXmlReader> = None;
        CreateXmlReader(&IXmlReader::IID, &mut reader as *mut _ as _, None)?;
        let reader = reader.unwrap();
        reader.SetInput(&stream)?;

        let mut name = PCWSTR::null();
        let mut name_len = 0;

        let mut node_type = XmlNodeType_None;
        reader.Read(Some(&mut node_type)).ok()?;
        assert_eq!(node_type, XmlNodeType_Element);
        reader.GetLocalName(&mut name, Some(&mut name_len))?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "html");

        assert_eq!(reader.GetAttributeCount()?, 2);
        reader.MoveToFirstAttribute().ok()?;

        reader.GetLocalName(&mut name, Some(&mut name_len))?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "no-value");

        reader.GetValue(&mut name, Some(&mut name_len))?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "");

        reader.MoveToNextAttribute().ok()?;

        reader.GetLocalName(&mut name, Some(&mut name_len))?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "with-value");

        reader.GetValue(&mut name, Some(&mut name_len))?;
        assert_eq!(String::from_utf16_lossy(std::slice::from_raw_parts(name.0, name_len as _)), "value");

        Ok(())
    }
}
