extern crate yaml_rust;

#[cfg(test)]
mod aliases {
    use yaml_rust::yaml;
    use yaml_rust::yaml::Yaml;

    #[allow(dead_code)]
    fn docs() -> Vec<Yaml>{
        let doc =
r#"---
- alias: &id001      # defines anchor label &id001
    instrument:      Lasik 2000

- alias: &id002
    instrument:      Lasik 2000

- step: *id001                   # refers to the first step (with anchor &id001)
- step: *id002                   # refers to the second step
"#;

        yaml::YamlLoader::load_from_str(&doc).unwrap()
    }

    #[test]
    fn aliases() {
        let docs = docs();

        let array = docs[0].as_vec().unwrap();
        assert_eq!(array[0], array[1]);
        let group0:Vec<Yaml> = array[0].as_hash().unwrap()
            .values().cloned().collect();

        let group1:Vec<Yaml> = array[1].as_hash().unwrap()
            .values().cloned().collect();

        let group2:Vec<Yaml> = array[2].as_hash().unwrap()
            .values().cloned().collect();

        println!("{:#?}", group0.get(0) );
        println!("{:#?}", group2.get(0) );
        assert_eq!(group0.get(0), group1.get(0));
        assert_eq!(group0.get(0), group2.get(0)); // failes due to missing alias handling

    }
}
