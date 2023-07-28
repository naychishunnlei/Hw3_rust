use assert_cmd::Command;

#[test]
fn pattern1_without_size(){
    let size = "";

    let expected = "";
    Command::cargo_bin("pattern1").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size1(){
    let size = "1";

    let expected = "*\n";
    Command::cargo_bin("pattern1").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size2(){
    let size = "2";

    let expected = "*\n**\n*\n";
    Command::cargo_bin("pattern1").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size3(){
    let size = "3";

    let expected = "*\n**\n***\n**\n*\n";

    Command::cargo_bin("pattern1").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size4(){
    let size = "4";

    let expected = "*\n**\n***\n****\n***\n**\n*\n";

    Command::cargo_bin("pattern1").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern1_with_size5(){
    let size = "5";

    let expected = "*\n**\n***\n****\n*****\n****\n***\n**\n*\n";

    Command::cargo_bin("pattern1").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

// ------------------------ Test for Star Pattern 2 -----------------------

#[test]
fn pattern2_without_size(){
    let size = "";

    let expected = "";
    Command::cargo_bin("pattern2").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size1(){
    let size = "1";

    let expected = "*\n";
    Command::cargo_bin("pattern2").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size2(){
    let size = "2";

    let expected = " *\n**\n *\n";
    Command::cargo_bin("pattern2").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size3(){
    let size = "3";

    let expected = "  *\n **\n***\n **\n  *\n";

    Command::cargo_bin("pattern2").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size4(){
    let size = "4";

    let expected = "   *\n  **\n ***\n****\n ***\n  **\n   *\n";

    Command::cargo_bin("pattern2").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}

#[test]
fn pattern2_with_size5(){
    let size = "5";

    let expected = "    *\n   **\n  ***\n ****\n*****\n ****\n  ***\n   **\n    *\n";

    Command::cargo_bin("pattern2").unwrap()
            .arg(size)
            .assert()
            .success().stdout(expected);
}
