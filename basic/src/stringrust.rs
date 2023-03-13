pub fn stringrust() {
    let var2 = "hello \
    \"rust\" \
    and \
    \"world\"";
    println!("{}", var2);

    let var3 = "baris satu
        baris dua
        baris tiga
    ";

    let var4 = "baris satu
    baris dua
    baris tiga";
    println!("{}", var4);

    println!("{}", var3);

    let var5 = r#"
    {
        "name": "tim drake",
        "gender": "male"
    }
    "#;
    println!("{}", var5);

    let var6 = "
    {
        \"name\": \"cassandra cain\",
        \"gender\": \"female\"
    }
    ";
    println!("{}", var6);
}
