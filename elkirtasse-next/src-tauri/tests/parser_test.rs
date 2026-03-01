use elkirtasse_next_lib::parser;

#[test]
fn test_parse_library() {
    let xml = r#"<dataroot>
        <root Name="Main">
            <Item Name="Group1" id="1">
                <bk id="bk1" name="Book1" aut="Author1" betaka="Info1" tfsr="0"/>
            </Item>
        </root>
    </dataroot>"#;
    let groups = parser::parse_library(xml).unwrap();
    assert_eq!(groups.len(), 1);
}
