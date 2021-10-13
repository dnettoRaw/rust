// Disable warnings
#[allow(unused_macros)]

//====================================================================
// flag zone 

// 'Flag' is in real world one byte active 1 or not 0 inside one variable 
// exemple u8 can put inside 8 'flag' 
// i use unsigned int because signe -/+ take one byte to be recorded
// in bynary we can see that 0b00000001 ( is 1 in decimal) 
//  exemple function :
/**
macro_rules! flag_set {($var:expr, $flag:expr) => {if ($var & $flag) == 0 {$var += $flag}}}
macro_rules! flag_test {($sele:expr, $base:expr) => { ($sele & $base) == 0};($sele:expr, $flag:expr, $($rest:expr),+) => { (( $sele & $flag) == 0 ) || flag_test!($sele, $($rest),+) };}

fn main() {
    const MY_VALUE : [&str; 2] = ["IS_OP1","OP9"];
    let mut selector = 0;
    for input in MY_VALUE {
        match input {
            // here the function take only the first arg and skip doublons
            x if x == "IS_OP1"  => { if flag_test!(selector, 2>>0) { flag_set!(selector, 1>>0)}} ,
            x if x == "IS_OP2"  => { if flag_test!(selector, 1>>0) { flag_set!(selector, 2>>0)}} ,
           _ => println!("Input [{}] does not equal any valable value", input),
        }
    }
    println!("{:#010b},{0}", selector);
}
 * 
 */
macro_rules! flag_set {
    // this macro is to keep cleans flag set this skip doublons
    ($var:expr, $flag:expr) => {if ($var & $flag) == 0 {$var += $flag}}
}]
macro_rules! flag_test {
    // this macro is to test if the flag is active in my variable, recomended to use inside 'if' like {if flag_test(myVar, flag1, ... ) {println!{"my Flag is active"}}}

    // this function take the 2 args and compare to 0 if true 
    ($sele:expr, $base:expr) => { ($sele & $base) == 0};
    // this function can take how many flags to teste if is in first arg       (Stack call /!\)
    ($sele:expr, $flag:expr, $($rest:expr),+) => { (( $sele & $flag) == 0 ) || flag_test!($sele, $($rest),+) };
}

//====================================================================
// debug print zone 
#[macro_export]
#[cfg(feature = "verbose")]
macro_rules! debug_print {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

#[macro_export]
#[cfg(not(feature = "verbose"))]
macro_rules! debug_print {
    ($( $args:expr ),*) => {println!( $( $args ),* ); }