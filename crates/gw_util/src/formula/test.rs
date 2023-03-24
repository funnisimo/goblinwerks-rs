use super::calculate;
use super::parse_formula;
use crate::formula::types::Error;
use crate::formula::NoCustomFunction;
use crate::formula::NoReference;
use crate::value::Value;

#[test]
fn const_value() {
    let formula = parse_formula::parse_string_to_formula(&"=1");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();

    assert_eq!(result, 1.0.into());
    // println!("Result is {}", calculate::result_to_number(result).unwrap());
}

fn data_function(s: String) -> Option<Value> {
    match s.as_str() {
        "TEXT" => Some(Value::String("TEXT".to_string())),
        "FLOAT" => Some(Value::Float(3.2)),
        "TEXT_FLOAT" => Some(Value::String("3.2".to_string())),
        "B" => Some(Value::Empty),
        "T" => Some(Value::Boolean(true)),
        "F" => Some(Value::Boolean(false)),
        "ReferenceKey" => Some(Value::String("100".to_string())),
        "ReferenceName" => Some(Value::String("Test".to_string())),

        // _ => Value::Error(Error::Value),
        _ => None,
    }
}

fn custom_functions(s: String, params: Vec<Value>) -> Result<Value, Error> {
    match s.as_str() {
        "ONE" => Ok(Value::Float(params[0].as_float().unwrap() + 1.0)),
        "TWO" => Ok(Value::Float(
            params[0].as_float().unwrap() + params[1].as_float().unwrap(),
        )),
        "Increase" => Ok(Value::Float(params[0].as_float().unwrap() + 1.0)),
        "SimpleSum" => Ok(Value::Float(
            params[0].as_float().unwrap() + params[1].as_float().unwrap(),
        )),
        "EqualFive" => Ok(Value::Float(5.0)),
        "BLANK" => Ok(Value::Empty),
        // _ => Value::Error(Error::Value),
        _ => {
            println!("ERROR - Uknexpected function request - {}", s);
            Err(Error::Argument)
        }
    }
}

#[test]
fn reference_text() {
    let formula = parse_formula::parse_string_to_formula(&"=TEXT");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, "TEXT".into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn reference_float() {
    let formula = parse_formula::parse_string_to_formula(&"=FLOAT");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, 3.2.into());
    // println!("Result is {}", calculate::result_to_number(result).unwrap());
}

#[test]
fn custom_func() {
    let formula = parse_formula::parse_string_to_formula(&"=ONE(1)");
    // println!("=ONE(1) > {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), Some(&data_function))
            .unwrap();
    assert_eq!(result, 2.0.into());
    // println!("Result is {}", calculate:s:result_to_string(result));
}

#[test]
fn two_param_func() {
    let formula = parse_formula::parse_string_to_formula(&"=TWO(1.2,2.1)");
    // println!("=TWO(1.2, 2.1) > {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), Some(&data_function))
            .unwrap();
    assert_eq!(result, 3.3.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn formula_with_ref() {
    let formula = parse_formula::parse_string_to_formula(&"=ONE(FLOAT)");
    // println!("=ONE(B) > {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), Some(&data_function))
            .unwrap();
    assert_eq!(result, 4.2.into());
    // println!(
    //     "Result [=ONE(B)] is {}",
    //     calculate::result_to_string(result)
    // );
}

#[test]
fn text_plus_float() {
    let formula = parse_formula::parse_string_to_formula(&"=TEXT+FLOAT");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    assert_eq!(result, Err(Error::Cast)); // string(text) + float = error
                                          // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn text_float_plus_float() {
    let formula = parse_formula::parse_string_to_formula(&"=TEXT_FLOAT+FLOAT");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, 6.4.into()); // string(float) + float = error
                                    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn missing_ref() {
    let formula = parse_formula::parse_string_to_formula(&"=SUM(A,B,C)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
    assert_eq!(result, Err(Error::Reference));
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn plus_float() {
    let formula = parse_formula::parse_string_to_formula(&"=1+2");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, 3.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn parens() {
    let formula = parse_formula::parse_string_to_formula(&"=(1*(2+3))*2");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, 10.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn div_zero() {
    let formula = parse_formula::parse_string_to_formula(&"=1+3/0"); // error (#DIV/0!)
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    assert_eq!(result, Err(Error::Div0));
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn string_concat() {
    let formula = parse_formula::parse_string_to_formula(&r#"="Hello" & " World!""#);
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "Hello World!".into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn float_plus_text() {
    let formula = parse_formula::parse_string_to_formula(&r#"=1 + "Hello""#); // error (#CAST!)
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    assert_eq!(result, Err(Error::Cast));
    // println!("Result is {}", calculate::resulst_to_string(result));
}

#[test]
fn no_equals() {
    let formula = parse_formula::parse_string_to_formula(&"1.2");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "1.2".into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"Hello World");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "Hello World".into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn abs_val() {
    let formula = parse_formula::parse_string_to_formula(&"=ABS(-1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, 1.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn sum() {
    let formula = parse_formula::parse_string_to_formula(&r#"=SUM(1,2,"3")"#);
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, 6.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn product() {
    let formula = parse_formula::parse_string_to_formula(&"=PRODUCT(ABS(1),2*1, 3,4*1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, 24.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn float_ge_float() {
    let formula = parse_formula::parse_string_to_formula(&"=2>=1");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, true.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn bool_or() {
    let formula = parse_formula::parse_string_to_formula(&"=OR(1>1,1!=1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, false.into());
    // println!("Result is {}", calculate::rsesult_to_string(result));
}

#[test]
fn bool_and() {
    let formula = parse_formula::parse_string_to_formula(&r#"=AND("test","false", 1, true) "#);
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, true.into());
    // println!("Result is {}", calculate::sresult_to_string(result));
}

#[test]
fn sum_list() {
    let formula = parse_formula::parse_string_to_formula(&"=SUM([1,2,3], 4, [5,6,7])");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, 28.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn average_list() {
    let formula = parse_formula::parse_string_to_formula(&"=AVERAGE([1,2,3],1,2,3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, 2.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn xor() {
    let formula = parse_formula::parse_string_to_formula(&"=XOR([0,0,0])");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, false.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn list_add() {
    let formula = parse_formula::parse_string_to_formula(&"=[1,2,3]+[1,2,3]");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, Value::from_array(&[2.0, 4.0, 6.0]));
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn list_add_wrong_size() {
    let formula = parse_formula::parse_string_to_formula(&"=[0,0]+[1,2,3]");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>);
    assert_eq!(result, Err(Error::Argument));
    // println!("Result is {}", calculate::result_to_string(result)); // error (#ARG!)
}

// // let start: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-03-01T02:00:00.000Z")?;
// // let end: DateTime<FixedOffset> = DateTime::parse_from_rfc3339("2019-08-30T02:00:00.000Z")?;
// // let data_function = |s: String| match s.as_str() {
// //     "start" => types::Value::Date(start),
// //     "end" => types::Value::Date(end),
// //     _ => types::Value::Error(types::Error::Value),
// // };

// // let formula = parse_formula::parse_string_to_formula(&"=DAYS(end, start)");
// // let result =
// //     calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function));
// // println!("Result is {}", calculate::result_to_string(result));

// let formula = parse_formula::parse_string_to_formula(&"=start+1");
// let result =
//     calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))?;
// println!("Result is {}", calculate::result_to_string(result));

// let formula = parse_formula::parse_string_to_formula(&"=end-3");
// let result =
//     calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))?;
// println!("Result is {}", calculate::result_to_string(result));

#[test]
fn formula() {
    let formula = parse_formula::parse_string_to_formula(&"=Increase(1)+1");
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>)
            .unwrap();
    assert_eq!(result, 3.0.into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=EqualFive()+1");
    // println!("Formula[=EqualFive()+1] = {:?}", formula);
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>)
            .unwrap();
    assert_eq!(result, 6.0.into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SimpleSum(1,2)");
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>)
            .unwrap();
    assert_eq!(result, 3.0.into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=EqualFive()");
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>)
            .unwrap();
    assert_eq!(result, 5.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

// ///////////// RIGHT function
#[test]
fn right_fn() {
    let formula = parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\", 3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "ple".into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=RIGHT(\"apple\")");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "e".into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&r#"="P"&RIGHT("000"&1,3)"#);
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "P001".into());
    // println!("Result is {}", calculate::result_to_string(result));
}

// ///////////// LEFT function
#[test]
fn left_fn() {
    let formula = parse_formula::parse_string_to_formula(&r#"=LEFT("apple", 3)"#);
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "app".into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&r#"=LEFT("apple")"#);
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, None::<NoReference>)
            .unwrap();
    assert_eq!(result, "a".into());
    // println!("Result is {}", calculate::result_to_string(result));
}

// ///////////// Handle blank in calculation

#[test]
fn empty_values() {
    let formula = parse_formula::parse_string_to_formula(&"=SUM(B, 1)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, 1.0.into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM(BLANK(), 1)");
    let result =
        calculate::calculate_formula(formula, Some(&custom_functions), None::<NoReference>)
            .unwrap();
    assert_eq!(result, 1.0.into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=OR([F,B])");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, false.into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula = parse_formula::parse_string_to_formula(&"=SUM(1, 2, , 3)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, 6.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

// ///////////// IF function
#[test]
fn if_fn() {
    let formula = parse_formula::parse_string_to_formula(&"=IF(TRUE,1,0)");
    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, 1.0.into());
    // println!("Result is {}", calculate::result_to_string(result));

    let formula =
        parse_formula::parse_string_to_formula(&r#"=IF(ReferenceKey=="10", "", ReferenceKey)"#);

    println!("FORMULA = {:?}", formula);

    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, "100".into());
    // println!("Result is {}", calculate::result_to_string(result));
}

#[test]
fn if_fn_formula() {
    // TODO - FOR SOME REASON PARENS ARE REQUIRED TO MAKE THIS WORK!!!
    let formula = parse_formula::parse_string_to_formula(&r#"=IF(FLOAT!=3.2, 2, (1 + 1 + 1) )"#);

    println!("FORMULA = {:?}", formula);

    let result =
        calculate::calculate_formula(formula, None::<NoCustomFunction>, Some(&data_function))
            .unwrap();
    assert_eq!(result, 3.0.into());
    // println!("Result is {}", calculate::result_to_string(result));
}

// println!("DONE");
