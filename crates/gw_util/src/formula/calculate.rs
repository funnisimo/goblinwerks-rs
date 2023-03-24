use super::parse_formula;
use super::types;
// use crate::{NoCustomFunction, NoReference};
// use chrono::{DateTime, FixedOffset};
// type NoCustomFunction<'a> = &'a fn(String, Vec<types::Value>) -> types::Value;

fn calculate_divide_operator(num1: f64, num2: f64) -> f64 {
    num1 / num2
}

fn is_float_int(num: f64) -> bool {
    //((num as i32) as f64) == num
    (((num as i32) as f64) - num).abs() == 0.0
}

fn calculate_power_operator(num1: f64, num2: f64) -> f64 {
    if is_float_int(num2) {
        num1.powi(num2 as i32)
    } else {
        num1.powf(num2)
    }
}

fn calculate_concat_operator(str1: &str, str2: &str) -> String {
    str1.to_owned() + str2
}

fn calculate_string_operation_rhs(
    l: &str,
    rhs: types::Value,
    f: fn(str1: &str, str2: &str) -> String,
) -> Result<types::Value, types::Error> {
    Ok(types::Value::String(f(&l, &rhs.to_string())))
    // match rhs {
    //     types::Value::Boolean(_) => rhs,
    //     types::Value::Float(r) => types::Value::String(f(&l, &r.to_string())),
    //     types::Value::String(r) => types::Value::String(f(&l, &r)),
    //     types::Value::List(_) => Err(types::Error::Value),
    //     // types::Value::Date(_) => Err(types::Error::Value),
    //     types::Value::Empty => types::Value::String(f(&l, "")),
    // }
}

fn calculate_string_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(str1: &str, str2: &str) -> String,
) -> Result<types::Value, types::Error> {
    calculate_string_operation_rhs(&lhs.to_string(), rhs, f)
    // match lhs {
    //     types::Value::Boolean(_) => lhs,
    //     // Err(_) => lhs,
    //     types::Value::Float(l) => calculate_string_operation_rhs(&l.to_string(), rhs, f),
    //     types::Value::String(l) => calculate_string_operation_rhs(&l, rhs, f),
    //     types::Value::List(_) => Err(types::Error::Value),
    //     // types::Value::Date(_) => Err(types::Error::Value),
    //     types::Value::Empty => calculate_string_operation_rhs("", rhs, f),
    // }
}

// fn calculate_numeric_operator_rhs_text(
//     t: String,
//     rhs: types::Value,
//     f: fn(num1: f64, num2: f64) -> f64,
// ) -> Result<types::Value, types::Error> {
//     match t.parse::<f64>() {
//         Ok(nl) => match rhs.as_float() {
//             Some(nr) => Ok(types::Value::Float(f(nl, nr))),
//             None => Err(types::Error::Value),
//         },
//         Err(_) => Err(types::Error::Cast),
//     }
// }

fn calculate_numeric_operator_rhs_number(
    _l: f64,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    match rhs {
        types::Value::List(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f)?;
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                Err(types::Error::Value)
            }
        }
        _ => match rhs.as_float() {
            None => Err(types::Error::Cast),
            Some(nr) => match lhs.as_float() {
                None => Err(types::Error::Cast),
                Some(nl) => Ok(types::Value::Float(f(nl, nr))),
            },
        },
    }
}

fn calculate_numeric_operator_product_rhs_number(
    l: f64,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    println!("calculate_numeric_operator_product_rhs_number");
    println!(" - lhs = {:?}", lhs);
    println!(" - rhs = {:?}", rhs);
    match rhs {
        types::Value::List(mut list) => {
            println!("- rhs is list");
            if let Some(mut temp) = list.pop() {
                println!("- temp={:?}", temp);
                while let Some(top) = list.pop() {
                    temp = calculate_numeric_product_operator(temp, top, f)?;
                    println!("- temp={:?}", temp);
                }
                calculate_numeric_product_operator(lhs, temp, f)
            } else {
                // Err(types::Error::Argument)
                Err(types::Error::Value)
            }
        }
        _ => match rhs.as_float() {
            None => Err(types::Error::Value),
            Some(nr) => Ok(types::Value::Float(f(l, nr))),
        },
    }

    // match rhs {
    //     types::Value::Boolean(_) => rhs,
    //     Err(_) => rhs,
    //     types::Value::String(t) => match t.parse::<f64>() {
    //         Ok(nr) => types::Value::Float(f(l, nr)),
    //         Err(_) => Err(types::Error::Cast),
    //     },
    //     types::Value::Float(r) => types::Value::Float(f(l, r)),
    //     types::Value::List(mut value_vec) => {
    //         if let Some(mut temp) = value_vec.pop() {
    //             while let Some(top) = value_vec.pop() {
    //                 temp = calculate_numeric_product_operator(temp, top, f);
    //             }
    //             calculate_numeric_product_operator(lhs, temp, f)
    //         } else {
    //             Err(types::Error::Argument)
    //         }
    //     }
    //     // types::Value::Date(_) => Err(types::Error::Value),
    //     types::Value::Empty => match lhs {
    //         types::Value::Empty => types::Value::Empty,
    //         _ => types::Value::Float(l),
    //     },
    // }
}

fn calculate_numeric_operator_rhs_iterator(
    mut lhs_vec: Vec<types::Value>,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    match rhs {
        types::Value::List(mut rhs_vec) => {
            let mut result_vec = Vec::new();
            loop {
                match (lhs_vec.pop(), rhs_vec.pop()) {
                    (Some(x), Some(y)) => {
                        result_vec.push(calculate_numeric_operator(x, y, f)?);
                    }
                    (Some(_), None) => return Err(types::Error::Argument),
                    (None, Some(_)) => return Err(types::Error::Argument),
                    (None, None) => break,
                };
            }
            Ok(types::Value::List(result_vec))
        }
        _ => match rhs.as_float() {
            None => Err(types::Error::Value),
            Some(nr) => {
                if let Some(mut temp) = lhs_vec.pop() {
                    while let Some(top) = lhs_vec.pop() {
                        temp = calculate_numeric_operator(temp, top, f)?;
                    }
                    calculate_numeric_operator(temp, rhs, f)
                } else {
                    // Err(types::Error::Argument)
                    Err(types::Error::Value)
                }
            }
        },
    }
}

// fn add_days_to_date(d: DateTime<FixedOffset>, rhs: types::Value) -> Result<types::Value,types::Error> {
//     match rhs {
//         types::Value::Float(x) => types::Value::Date(d + Duration::days(x as i64)),
//         _ => Err(types::Error::Value),
//     }
// }

// fn subtract_days_from_date(d: DateTime<FixedOffset>, rhs: types::Value) -> Result<types::Value,types::Error> {
//     match rhs {
//         types::Value::Float(x) => types::Value::Date(d - Duration::days(x as i64)),
//         _ => Err(types::Error::Value),
//     }
// }

fn calculate_numeric_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    //println!("{:?}::{:?}", lhs, rhs);
    match lhs {
        // types::Value::Boolean(_) => lhs,
        // // Err(_) => lhs,
        // types::Value::String(t) => calculate_numeric_operator_rhs_text(t, rhs, f),
        // types::Value::Float(l) => calculate_numeric_operator_rhs_number(l, lhs, rhs, f),
        // types::Value::Integer(l) => calculate_numeric_operator_rhs_number(l as f64, lhs, rhs, f),
        types::Value::List(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        // types::Value::Date(_) => Err(types::Error::Value),
        // types::Value::Empty => calculate_numeric_operator_rhs_number(0.0, lhs, rhs, f),
        _ => match lhs.as_float() {
            None => Err(types::Error::Cast),
            Some(nl) => calculate_numeric_operator_rhs_number(nl, lhs, rhs, f),
        },
    }
}

fn calculate_numeric_product_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    //println!("{:?}::{:?}", lhs, rhs);
    match lhs {
        // types::Value::Boolean(_) => lhs,
        // Err(_) => lhs,
        // types::Value::String(t) => calculate_numeric_operator_rhs_text(t, rhs, f),
        // types::Value::Float(l) => calculate_numeric_operator_product_rhs_number(l, lhs, rhs, f),
        types::Value::List(lhs_vec) => calculate_numeric_operator_rhs_iterator(lhs_vec, rhs, f),
        // types::Value::Date(_) => Err(types::Error::Value),
        // types::Value::Empty => calculate_numeric_operator_product_rhs_number(1.0, lhs, rhs, f),
        _ => match lhs.as_float() {
            None => Err(types::Error::Value),
            Some(nl) => calculate_numeric_operator_product_rhs_number(nl, lhs, rhs, f),
        },
    }
}

fn calculate_average_operator_rhs_number(
    element_count: &mut i32,
    l: f64,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    match rhs {
        // types::Value::Boolean(_) => rhs,
        // Err(_) => rhs,
        // types::Value::String(t) => match t.parse::<f64>() {
        //     Ok(nr) => types::Value::Float(f(l, nr)),
        //     Err(_) => Err(types::Error::Cast),
        // },
        // types::Value::Float(r) => types::Value::Float(f(l, r)),
        types::Value::List(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                match temp {
                    types::Value::Empty => *element_count -= 1,
                    _ => (),
                };
                while let Some(top) = value_vec.pop() {
                    temp = calculate_numeric_operator(temp, top.clone(), f)?;
                    match top {
                        types::Value::Empty => (),
                        _ => *element_count += 1,
                    };
                }
                calculate_numeric_operator(lhs, temp, f)
            } else {
                // Err(types::Error::Argument)
                Err(types::Error::Value)
            }
        }
        // types::Value::Date(_) => Err(types::Error::Value),
        types::Value::Empty => {
            *element_count -= 1;
            Ok(types::Value::Float(f(l, 0.0)))
        }
        _ => match rhs.as_float() {
            None => Err(types::Error::Value),
            Some(nr) => Ok(types::Value::Float(f(l, nr))),
        },
    }
}

fn calculate_average_operator_rhs_iterator(
    element_count: &mut i32,
    mut lhs_vec: Vec<types::Value>,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    match rhs.is_float() {
        true => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_numeric_operator(temp, top, f)?;
                    *element_count += 1;
                }
                calculate_numeric_operator(temp, rhs, f)
            } else {
                // Err(types::Error::Argument)
                Err(types::Error::Value)
            }
        }
        // _ => Err(types::Error::Value),
        false => Err(types::Error::Value),
    }
}

fn calculate_average_operator(
    element_count: &mut i32,
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    match lhs {
        // types::Value::Boolean(_) => lhs,
        // Err(_) => lhs,
        // types::Value::String(t) => calculate_numeric_operator_rhs_text(t, rhs, f),
        // types::Value::Float(l) => {
        //     calculate_average_operator_rhs_number(element_count, l, lhs, rhs, f)
        // }
        types::Value::List(lhs_vec) => {
            calculate_average_operator_rhs_iterator(element_count, lhs_vec, rhs, f)
        }
        // types::Value::Date(_) => Err(types::Error::Value),
        types::Value::Empty => {
            *element_count -= 1;
            calculate_average_operator_rhs_number(element_count, 0.0, lhs, rhs, f)
        }
        _ => match lhs.as_float() {
            None => Err(types::Error::Value),
            Some(nl) => calculate_average_operator_rhs_number(element_count, nl, lhs, rhs, f),
        },
    }
}

fn calculate_comparison_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(num1: f64, num2: f64) -> bool,
) -> Result<types::Value, types::Error> {
    println!("compare - {:?} vs {:?}", lhs, rhs);
    match lhs {
        types::Value::String(l) => match rhs {
            types::Value::String(r) => {
                let r = if l.eq(&r) {
                    f(1.0.into(), 1.0.into())
                } else if l.lt(&r) {
                    f(0.0.into(), 1.0.into())
                } else {
                    f(1.0.into(), 0.0.into())
                };
                println!("- {}", r);
                Ok(types::Value::Boolean(r))
            }
            // _ => Err(types::Error::Value),
            _ => Err(types::Error::Value),
        },
        types::Value::Float(l) => match rhs {
            types::Value::Float(r) => Ok(types::Value::Boolean(f(l, r))),
            types::Value::Empty => Ok(types::Value::Boolean(f(l, 0.0))),
            // _ => Err(types::Error::Value),
            _ => Err(types::Error::Value),
        },
        types::Value::Empty => match rhs {
            types::Value::Float(r) => Ok(types::Value::Boolean(f(0.0, r))),
            types::Value::Empty => Ok(types::Value::Boolean(true)),
            // _ => Err(types::Error::Value),
            _ => Err(types::Error::Value),
        },
        // types::Value::Boolean(_) => Err(types::Error::Value),
        // Err(_) => Err(types::Error::Value),
        // types::Value::List(_) => Err(types::Error::Value),
        // types::Value::Date(_) => Err(types::Error::Value),
        _ => Err(types::Error::Value),
    }
}

fn calculate_boolean_operator_rhs_boolean(
    l: bool,
    rh: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    match rh {
        types::Value::Boolean(r) => {
            if f(l, r) {
                Ok(types::Value::Boolean(true))
            } else {
                Ok(types::Value::Boolean(false))
            }
        }
        // Err(_) => match l {
        //     true => Ok(types::Value::Boolean(true)),
        //     false => Ok(types::Value::Boolean(false)),
        // },
        types::Value::List(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f)?;
                }
                let rhs = cast_value_to_boolean(temp)?;
                match rhs {
                    types::Value::Boolean(r) => {
                        if f(l, r) {
                            Ok(types::Value::Boolean(true))
                        } else {
                            Ok(types::Value::Boolean(false))
                        }
                    }
                    _ => Err(types::Error::Value),
                }
            } else {
                Err(types::Error::Argument)
            }
        }
        types::Value::Empty => {
            if l {
                Ok(types::Value::Boolean(true))
            } else {
                Ok(types::Value::Boolean(false))
            }
        }
        _ => Err(types::Error::Value),
    }
}

fn calculate_boolean_operator_rhs_error(rh: types::Value) -> Result<types::Value, types::Error> {
    match rh {
        types::Value::Boolean(r) => {
            if r {
                Ok(types::Value::Boolean(true))
            } else {
                Ok(types::Value::Boolean(false))
            }
        }
        // Err(_) => Err(types::Error::Cast),
        _ => Err(types::Error::Value),
    }
}

fn calculate_boolean_operator_rhs_iterator(
    rh: types::Value,
    mut lhs_vec: Vec<types::Value>,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    match rh {
        types::Value::Boolean(r) => {
            if let Some(mut temp) = lhs_vec.pop() {
                while let Some(top) = lhs_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f)?;
                }
                let lhs = cast_value_to_boolean(temp)?;
                match lhs {
                    types::Value::Boolean(l) => {
                        if f(l, r) {
                            Ok(types::Value::Boolean(true))
                        } else {
                            Ok(types::Value::Boolean(false))
                        }
                    }
                    _ => Err(types::Error::Value),
                }
            } else {
                Err(types::Error::Argument)
            }
        }

        _ => Err(types::Error::Value),
    }
}

fn calculate_boolean_operator(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    let lh = cast_value_to_boolean(lhs)?;
    let rh = cast_value_to_boolean(rhs)?;
    match lh {
        types::Value::Boolean(l) => calculate_boolean_operator_rhs_boolean(l, rh, f),
        // Err(_) => calculate_boolean_operator_rhs_error(rh),
        types::Value::List(lhs_vec) => calculate_boolean_operator_rhs_iterator(rh, lhs_vec, f),
        types::Value::Empty => calculate_boolean_operator_rhs_boolean(true, rh, f),
        _ => Err(types::Error::Value),
    }
}

fn calculate_boolean_operator_or(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    let lh = cast_value_to_boolean(lhs)?;
    let rh = cast_value_to_boolean(rhs)?;

    match lh {
        types::Value::Boolean(l) => calculate_boolean_operator_rhs_boolean(l, rh, f),
        // Err(_) => calculate_boolean_operator_rhs_error(rh),
        types::Value::List(lhs_vec) => calculate_boolean_operator_rhs_iterator(rh, lhs_vec, f),
        types::Value::Empty => calculate_boolean_operator_rhs_boolean(false, rh, f),
        _ => Err(types::Error::Value),
    }
}

fn calculate_boolean_operator_xor(
    lhs: types::Value,
    rhs: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    let lh = cast_value_to_boolean(lhs)?;
    let rh = cast_value_to_boolean(rhs)?;

    match lh {
        types::Value::Boolean(l) => calculate_boolean_operator_rhs_boolean(l, rh, f),
        // Err(_) => calculate_boolean_operator_rhs_error(rh),
        types::Value::List(lhs_vec) => calculate_boolean_operator_rhs_iterator(rh, lhs_vec, f),
        types::Value::Empty => calculate_boolean_operator_rhs_boolean(false, rh, f),
        _ => Err(types::Error::Value),
    }
}

fn calculate_abs(value: types::Value) -> Result<types::Value, types::Error> {
    match value {
        // types::Value::Boolean(_) => Ok(value),
        // Err(_) => value,
        // types::Value::String(_) => Ok(value),
        types::Value::Float(l) => Ok(types::Value::Float(l.abs())),
        types::Value::Integer(l) => Ok(types::Value::Integer(l.abs())),
        // types::Value::List(_) => Err(types::Error::Value),
        // types::Value::Date(_) => Err(types::Error::Value),
        types::Value::Empty => Ok(types::Value::Float(0.0)),
        _ => Err(types::Error::Value),
    }
}

fn calculate_negation(value: types::Value) -> Result<types::Value, types::Error> {
    match value {
        types::Value::Boolean(l) => {
            if !l {
                Ok(types::Value::Boolean(true))
            } else {
                Ok(types::Value::Boolean(false))
            }
        }
        // Err(_) => value,
        types::Value::String(t) => {
            let l = cast_text_to_boolean(&t);
            match l {
                Some(l) => {
                    if !l {
                        Ok(types::Value::Boolean(true))
                    } else {
                        Ok(types::Value::Boolean(false))
                    }
                }
                None => Err(types::Error::Cast),
            }
        }
        types::Value::Float(l) => {
            if l == 0.0 {
                Ok(types::Value::Boolean(true))
            } else {
                Ok(types::Value::Boolean(false))
            }
        }
        types::Value::Integer(l) => {
            if l == 0 {
                Ok(types::Value::Boolean(true))
            } else {
                Ok(types::Value::Boolean(false))
            }
        }
        // types::Value::List(_) => Err(types::Error::Value),
        // types::Value::Date(_) => Err(types::Error::Value),
        types::Value::Empty => Ok(types::Value::Boolean(true)),
        _ => Err(types::Error::Value),
    }
}

fn calculate_negate(value: types::Value) -> Result<types::Value, types::Error> {
    match value {
        types::Value::Float(l) => Ok(types::Value::Float(-l)),
        types::Value::Integer(l) => Ok(types::Value::Integer(-l)),
        types::Value::List(mut value_vec) => {
            let mut result_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                result_vec.push(calculate_negate(top)?);
            }
            Ok(types::Value::List(result_vec))
        }
        types::Value::Empty => Ok(types::Value::Empty),
        types::Value::Boolean(b) => Ok(types::Value::Boolean(!b)),
        _ => Err(types::Error::Value),
    }
}

fn cast_text_to_boolean(s: &str) -> Option<bool> {
    if s.eq_ignore_ascii_case("TRUE") {
        Some(true)
    } else if s.eq_ignore_ascii_case("FALSE") {
        Some(false)
    } else {
        None
    }
}

fn cast_value_to_boolean(value: types::Value) -> Result<types::Value, types::Error> {
    match value {
        // types::Value::Boolean(_) => Ok(value),
        // Err(_) => value,
        // types::Value::String(t) => {
        //     let l = cast_text_to_boolean(&t);
        //     match l {
        //         Some(l) => {
        //             if l {
        //                 types::Value::Boolean(true)
        //             } else {
        //                 types::Value::Boolean(false)
        //             }
        //         }
        //         None => Err(types::Error::Cast),
        //     }
        // }
        // types::Value::Float(l) => {
        //     if l != 0.0 {
        //         types::Value::Boolean(true)
        //     } else {
        //         types::Value::Boolean(false)
        //     }
        // }
        types::Value::List(mut value_vec) => {
            let mut boolean_vec = Vec::new();
            while let Some(top) = value_vec.pop() {
                let value = cast_value_to_boolean(top)?;
                boolean_vec.push(value);
            }
            Ok(types::Value::List(boolean_vec))
        }
        // types::Value::Date(_) => Err(types::Error::Cast),
        types::Value::Empty => Ok(types::Value::Empty),
        _ => match value.as_bool() {
            None => Err(types::Error::Cast),
            Some(v) => Ok(types::Value::Boolean(v)),
        },
    }
}

fn convert_iterator_to_result(
    result: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    match result {
        types::Value::List(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator(temp, top, f)?;
                }
                match cast_value_to_boolean(temp)? {
                    types::Value::Boolean(bool_result) => {
                        if bool_result {
                            Ok(types::Value::Boolean(true))
                        } else {
                            Ok(types::Value::Boolean(false))
                        }
                    }
                    _ => Err(types::Error::Value),
                }
            } else {
                Err(types::Error::Argument)
            }
        }
        _ => Ok(result),
    }
}

fn convert_iterator_to_result_or(
    result: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    match result {
        types::Value::List(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator_or(temp, top, f)?;
                }
                match cast_value_to_boolean(temp)? {
                    types::Value::Boolean(bool_result) => {
                        if bool_result {
                            Ok(types::Value::Boolean(true))
                        } else {
                            Ok(types::Value::Boolean(false))
                        }
                    }
                    _ => Err(types::Error::Value),
                }
            } else {
                Err(types::Error::Argument)
            }
        }
        _ => Ok(result),
    }
}

fn convert_iterator_to_result_xor(
    result: types::Value,
    f: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    match result {
        types::Value::List(mut value_vec) => {
            if let Some(mut temp) = value_vec.pop() {
                while let Some(top) = value_vec.pop() {
                    temp = calculate_boolean_operator_xor(temp, top, f)?;
                }
                match cast_value_to_boolean(temp)? {
                    types::Value::Boolean(bool_result) => {
                        if bool_result {
                            Ok(types::Value::Boolean(true))
                        } else {
                            Ok(types::Value::Boolean(false))
                        }
                    }
                    _ => Err(types::Error::Value),
                }
            } else {
                Err(types::Error::Argument)
            }
        }
        _ => Ok(result),
    }
}

fn get_values(
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<(types::Value, types::Value), types::Error> {
    Ok((
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f, r)?,
            None => return Err(types::Error::Argument),
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f, r)?,
            None => return Err(types::Error::Argument),
        },
    ))
}

fn get_value(
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<types::Value, types::Error> {
    match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f, r),
        None => Err(types::Error::Argument),
    }
}

// fn get_date_values(
//     mut exp: types::Expression,
//     f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value,types::Error>>,
//     r: Option<&impl Fn(String) -> Option<types::Value>>,
// ) -> Result<(types::Value, types::Value),types::Error> {
//     Ok((
//         match exp.values.pop() {
//             Some(formula) => calculate_formula(formula, f, r)?,
//             None => return Err(types::Error::Argument),
//         },
//         match exp.values.pop() {
//             Some(formula) => calculate_formula(formula, f, r)?,
//             None => return Err(types::Error::Argument),
//         },
//     ))
// }

fn get_number_and_string_values(
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<(types::Value, types::Value), types::Error> {
    if exp.values.len() == 1 {
        Ok((
            types::Value::Float(1.0),
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f, r)?,
                None => return Err(types::Error::Argument),
            },
        ))
    } else {
        Ok((
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f, r)?,
                None => return Err(types::Error::Argument),
            },
            match exp.values.pop() {
                Some(formula) => calculate_formula(formula, f, r)?,
                None => return Err(types::Error::Argument),
            },
        ))
    }
}

fn get_iff_values(
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<(types::Value, types::Value, types::Value), types::Error> {
    println!("get_iff_values: {:?}", exp.values);
    Ok((
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f, r)?,
            None => types::Value::Empty,
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f, r)?,
            None => types::Value::Empty,
        },
        match exp.values.pop() {
            Some(formula) => calculate_formula(formula, f, r)?,
            None => types::Value::Empty,
        },
    ))
}

fn calculate_iterator(
    mut vec: Vec<types::Formula>,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<types::Value, types::Error> {
    // println!("iterator");
    let mut value_vec = Vec::new();
    while let Some(top) = vec.pop() {
        value_vec.push(calculate_formula(top, f, r)?);
    }
    Ok(types::Value::List(value_vec))
}

fn calculate_reference(
    string: String,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<types::Value, types::Error> {
    println!("reference - {:?}", string);
    match r {
        Some(r) => match r(string) {
            Some(v) => {
                println!("- {:?}", v);
                match v {
                    // types::Value::Float(x) => Ok(types::Value::Float(x)),
                    // types::Value::Integer(x) => Ok(types::Value::Integer(x)),
                    types::Value::String(s) => {
                        calculate_formula(parse_formula::parse_string_to_formula(&s), f, Some(r))
                        // Ok(types::Value::String(s))
                    }
                    // types::Value::Boolean(x) => Ok(types::Value::Boolean(x)),
                    // // Err(types::Error::Value) => Err(types::Error::Value),
                    // types::Value::List(v) => Ok(types::Value::List(v)),
                    // // types::Value::Date(d) => types::Value::Date(d),
                    // types::Value::Empty => Ok(types::Value::Empty),
                    _ => Ok(v),
                }
            }
            None => Err(types::Error::Reference),
        },
        None => Err(types::Error::Reference),
    }
}

fn calculate_bool(
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f, r)?,
        None => return Err(types::Error::Argument),
    };
    result = cast_value_to_boolean(result)?;
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator(result, calculate_formula(top, f, r)?, f_bool)?;
    }
    convert_iterator_to_result(result, f_bool)
}

fn calculate_or(
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f, r)?,
        None => return Err(types::Error::Argument),
    };
    result = cast_value_to_boolean(result)?;
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator_or(result, calculate_formula(top, f, r)?, f_bool)?;
    }
    convert_iterator_to_result_or(result, f_bool)
}

fn calculate_xor(
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
    f_bool: fn(bool1: bool, bool2: bool) -> bool,
) -> Result<types::Value, types::Error> {
    let mut result = match exp.values.pop() {
        Some(formula) => calculate_formula(formula, f, r)?,
        None => return Err(types::Error::Argument),
    };
    result = cast_value_to_boolean(result)?;
    while let Some(top) = exp.values.pop() {
        result = calculate_boolean_operator_xor(result, calculate_formula(top, f, r)?, f_bool)?;
    }
    convert_iterator_to_result_xor(result, f_bool)
}

fn calculate_collective_operator(
    mut collective_value: types::Value,
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
    f_collective: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    while let Some(top) = exp.values.pop() {
        collective_value = calculate_numeric_operator(
            collective_value,
            calculate_formula(top, f, r)?,
            f_collective,
        )?;
    }
    Ok(collective_value)
}

fn calculate_collective_product_operator(
    mut collective_value: types::Value,
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
    f_collective: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    println!(
        "calculate_collective_product_operator - starting: {:?}",
        collective_value
    );
    while let Some(top) = exp.values.pop() {
        println!("- top: {:?}", top);
        collective_value = calculate_numeric_product_operator(
            collective_value,
            calculate_formula(top, f, r)?,
            f_collective,
        )?;
        println!("- - value={:?}", collective_value);
    }
    Ok(match collective_value {
        types::Value::Empty => types::Value::Float(0.0),
        _ => collective_value,
    })
}

fn calculate_average(
    mut collective_value: types::Value,
    mut exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
    f_collective: fn(num1: f64, num2: f64) -> f64,
) -> Result<types::Value, types::Error> {
    let mut element_count = 0;
    while let Some(top) = exp.values.pop() {
        element_count += 1;
        collective_value = calculate_average_operator(
            &mut element_count,
            collective_value,
            calculate_formula(top, f, r)?,
            f_collective,
        )?;
    }
    if element_count == 0 {
        Err(types::Error::Div0)
    } else {
        calculate_numeric_operator(
            collective_value,
            types::Value::Float(element_count as f64),
            calculate_divide_operator,
        )
    }
}

// fn calculate_days(date_pair: (types::Value, types::Value)) -> Result<types::Value,types::Error> {
//     let begin_of_date: DateTime<FixedOffset> =
//         DateTime::parse_from_rfc3339("1900-01-01T02:00:00.000Z")
//             .ok()
//             .unwrap();
//     let (start, end) = date_pair;
//     match (start, end) {
//         // (types::Value::Date(start), types::Value::Date(end)) => {
//         //     types::Value::Float((end - start).num_days() as f64)
//         // }
//         (types::Value::Empty, types::Value::Date(end)) => {
//             types::Value::Float((end - begin_of_date).num_days() as f64)
//         }
//         (types::Value::Date(start), types::Value::Empty) => {
//             types::Value::Float((begin_of_date - start).num_days() as f64)
//         }
//         (types::Value::Empty, types::Value::Empty) => types::Value::Float(0.0),
//         _ => Err(types::Error::Value),
//     }
// }

fn calculate_right(
    number_string: (types::Value, types::Value),
) -> Result<types::Value, types::Error> {
    let (number, string) = number_string;

    let trim_length = match number.as_int() {
        Some(x) => x as usize,
        _ => 0,
    };

    let trimmed_string = match string {
        types::Value::String(s) => s[(s.len() - trim_length)..].to_string(),
        _ => "".to_string(),
    };
    Ok(types::Value::String(trimmed_string))
}

fn calculate_left(
    number_string: (types::Value, types::Value),
) -> Result<types::Value, types::Error> {
    let (number, string) = number_string;
    let trim_length = match number.as_int() {
        Some(x) => x as usize,
        _ => 0,
    };

    let trimmed_string = match string {
        types::Value::String(s) => {
            let temp: &'static str = Box::leak(s.into_boxed_str());
            &temp[..trim_length]
        }
        _ => "",
    };
    Ok(types::Value::String(trimmed_string.to_string()))
}

fn calculate_iff(
    iff_arguments: (types::Value, types::Value, types::Value),
) -> Result<types::Value, types::Error> {
    let (false_value, true_value, bool_expression) = iff_arguments;

    println!(
        "IFF = {:?} ? {:?} : {:?}",
        bool_expression, true_value, false_value
    );

    match bool_expression.as_bool() {
        Some(true) => {
            println!("- true : [{:?}]", true_value);
            Ok(true_value)
        }
        Some(false) => {
            println!("- false : [{:?}]", false_value);
            Ok(false_value)
        }
        None => Err(types::Error::Argument),
    }
}

fn calculate_function(
    func: types::Function,
    exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<types::Value, types::Error> {
    // println!("calc func");
    match func {
        types::Function::Abs => calculate_abs(get_value(exp, f, r)?),
        types::Function::Sum => {
            calculate_collective_operator(types::Value::Float(0.0), exp, f, r, |n1, n2| n1 + n2)
        }
        types::Function::Product => {
            calculate_collective_product_operator(types::Value::Float(1.0), exp, f, r, |n1, n2| {
                n1 * n2
            })
        }
        types::Function::Average => {
            calculate_average(types::Value::Float(0.00), exp, f, r, |n1, n2| n1 + n2)
        }
        types::Function::Or => calculate_or(exp, f, r, |n1, n2| n1 || n2),
        types::Function::And => calculate_bool(exp, f, r, |n1, n2| n1 && n2),
        types::Function::Xor => calculate_xor(exp, f, r, |n1, n2| n1 ^ n2),
        types::Function::Not => calculate_negation(get_value(exp, f, r)?),
        types::Function::Negate => calculate_negate(get_value(exp, f, r)?),
        // types::Function::Days => calculate_days(get_date_values(exp, f, r)?),
        types::Function::Right => calculate_right(get_number_and_string_values(exp, f, r)?),
        types::Function::Left => calculate_left(get_number_and_string_values(exp, f, r)?),
        types::Function::Iff => calculate_iff(get_iff_values(exp, f, r)?),
        types::Function::Custom(name) => {
            let mut values: Vec<types::Value> = Vec::new();
            for value in exp.values.iter() {
                values.push(calculate_formula(value.clone(), f, r)?);
            }

            match f {
                Some(f) => match f(name, values) {
                    Ok(v) => match v {
                        types::Value::Float(x) => Ok(types::Value::Float(x)),
                        types::Value::String(s) => Ok(types::Value::String(s)),
                        types::Value::Boolean(x) => Ok(types::Value::Boolean(x)),
                        types::Value::List(v) => Ok(types::Value::List(v)),
                        // types::Value::Date(d) => types::Value::Date(d),
                        types::Value::Empty => Ok(types::Value::Empty),
                        _ => Err(types::Error::Reference),
                    },
                    Err(e) => Err(e),
                },
                None => Err(types::Error::Reference),
            }
        }
    }
}

fn calculate_operation(
    exp: types::Expression,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<types::Value, types::Error> {
    // println!("operation");
    match &exp.op {
        types::Operator::Plus => {
            let (value2, value1) = get_values(exp, f, r)?;
            match value1 {
                // types::Value::Date(d) => add_days_to_date(d, value2),
                _ => calculate_numeric_operator(value1, value2, |n1, n2| n1 + n2),
            }
        }

        types::Operator::Minus => {
            let (value2, value1) = get_values(exp, f, r)?;
            match value1 {
                // types::Value::Date(d) => subtract_days_from_date(d, value2),
                _ => calculate_numeric_operator(value1, value2, |n1, n2| n1 - n2),
            }
        }

        types::Operator::Multiply => {
            let (value2, value1) = get_values(exp, f, r)?;
            calculate_numeric_operator(value1, value2, |n1, n2| n1 * n2)
        }
        types::Operator::Divide => {
            let (value2, value1) = get_values(exp, f, r)?;
            match value2 {
                types::Value::Float(x) if x == 0.0 => Err(types::Error::Div0),
                _ => calculate_numeric_operator(value1, value2, calculate_divide_operator),
            }
        }
        types::Operator::Power => {
            let (value2, value1) = get_values(exp, f, r)?;
            calculate_numeric_operator(value1, value2, calculate_power_operator)
        }
        types::Operator::Concat => {
            let (value2, value1) = get_values(exp, f, r)?;
            println!("concat: {:?}, {:?}", value1, value2);
            let r = calculate_string_operator(value1, value2, calculate_concat_operator);
            println!(" - {:?}", r);
            r
        }
        types::Operator::Equal => {
            let (value2, value1) = get_values(exp, f, r)?;
            match (value1.clone(), value2.clone()) {
                // (types::Value::Date(l), types::Value::Date(r)) => {
                //     compare_dates(l, r, |d1, d2| d1 == d2)
                // }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| {
                    (n1 - n2).abs() < f64::EPSILON
                }),
            }
        }
        types::Operator::NotEqual => {
            let (value2, value1) = get_values(exp, f, r)?;
            match (value1.clone(), value2.clone()) {
                // (types::Value::Date(l), types::Value::Date(r)) => {
                //     compare_dates(l, r, |d1, d2| d1 != d2)
                // }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| {
                    (n1 - n2).abs() > f64::EPSILON
                }),
            }
        }
        types::Operator::Greater => {
            let (value2, value1) = get_values(exp, f, r)?;
            match (value1.clone(), value2.clone()) {
                // (types::Value::Date(l), types::Value::Date(r)) => {
                //     compare_dates(l, r, |d1, d2| d1 > d2)
                // }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 > n2),
            }
        }
        types::Operator::Less => {
            let (value2, value1) = get_values(exp, f, r)?;
            match (value1.clone(), value2.clone()) {
                // (types::Value::Date(l), types::Value::Date(r)) => {
                //     compare_dates(l, r, |d1, d2| d1 < d2)
                // }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 < n2),
            }
        }
        types::Operator::GreaterOrEqual => {
            let (value2, value1) = get_values(exp, f, r)?;
            match (value1.clone(), value2.clone()) {
                // (types::Value::Date(l), types::Value::Date(r)) => {
                //     compare_dates(l, r, |d1, d2| d1 >= d2)
                // }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 >= n2),
            }
        }
        types::Operator::LessOrEqual => {
            let (value2, value1) = get_values(exp, f, r)?;
            match (value1.clone(), value2.clone()) {
                // (types::Value::Date(l), types::Value::Date(r)) => {
                //     compare_dates(l, r, |d1, d2| d1 <= d2)
                // }
                _ => calculate_comparison_operator(value1, value2, |n1, n2| n1 <= n2),
            }
        }
        types::Operator::Function(func) => calculate_function(func.clone(), exp, f, r),
    }
}

// fn compare_dates(
//     date1: DateTime<FixedOffset>,
//     date2: DateTime<FixedOffset>,
//     f: fn(d1: DateTime<FixedOffset>, d2: DateTime<FixedOffset>) -> bool,
// ) -> Result<types::Value,types::Error> {
//     if f(date1, date2) {
//         Ok(types::Value::Boolean(true))
//     } else {
//         Ok(types::Value::Boolean(false))
//     }
// }

/// Evaluates a string that was parsed and stored in Expression Struct.
/// Takes an optional closure of custom functions with the trait bound Fn(String, Vec<types::Value>) -> types::Value.
/// Takes an optional closure of custom variables with the trait bound Fn(String) -> types::Value.
pub fn calculate_formula(
    formula: types::Formula,
    f: Option<&impl Fn(String, Vec<types::Value>) -> Result<types::Value, types::Error>>,
    r: Option<&impl Fn(String) -> Option<types::Value>>,
) -> Result<types::Value, types::Error> {
    println!("- calculate formula: {:?}", formula);
    match formula {
        types::Formula::Operation(exp) => calculate_operation(exp, f, r),
        types::Formula::Value(val) => {
            // println!("value");
            Ok(val)
        }
        types::Formula::Reference(string) => calculate_reference(string, f, r),
        types::Formula::Iterator(vec) => calculate_iterator(vec, f, r),
        types::Formula::Error(e) => Err(e),
    }
}

/// Converts a result from Value Enum to a printable string.
pub fn result_to_string(value: types::Value) -> String {
    value.to_string()
}

/// Converts a result from Value Enum to a printable string.
pub fn result_to_number(value: types::Value) -> Result<f64, types::Error> {
    match value {
        types::Value::Float(number) => Ok(number),
        _ => Err(types::Error::Argument),
    }
}

// fn show_number(number: f64) -> String {
//     if number.is_infinite() {
//         String::from("#DIV/0!")
//     } else {
//         number.to_string()
//     }
// }

// fn show_error(error: types::Error) -> String {
//     match error {
//         types::Error::Div0 => String::from("#DIV/0!"),
//         types::Error::Cast => String::from("#CAST!"),
//         types::Error::Parse => String::from("#PARSE!"),
//         types::Error::Value => String::from("#VALUE!"),
//         types::Error::Argument => String::from("#ARG!"),
//         types::Error::Reference => String::from("#REF!"),
//     }
// }

// fn show_boolean(boolean: bool) -> String {
//     match boolean {
//         true => String::from("TRUE"),
//         false => String::from("FALSE"),
//     }
// }

// fn show_iterator(mut value_vec: Vec<types::Value>) -> String {
//     value_vec.reverse();
//     let mut result = '{'.to_string();
//     while let Some(top) = value_vec.pop() {
//         result = result + &result_to_string(top);
//         result = result + &','.to_string();
//     }
//     result = result.trim_end_matches(',').to_string();
//     result = result + &'}'.to_string();
//     result
// }

// fn show_blank() -> String {
//     show_number(0.0)
//     //String::from("BLANK")
// }
