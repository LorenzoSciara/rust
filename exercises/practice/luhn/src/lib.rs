/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut pos = 1;
    let mut tmp;
    let mut result=0;
    if code.trim().len() <= 1 {
        return false;
    }
    for c in code.chars() {
        if c!=' ' {
            tmp= c.to_digit(10);
            match tmp {
                Some(x) => {
                    digits.push(x);
                    /*if pos % 2 != 0 {
                        if x*2 > 9 {
                            digits.push(x*2 - 9);
                        }
                        else {
                            digits.push(x*2);
                        }
                    } else {
                        digits.push(x);
                    }
                    pos = pos + 1;*/
                },
                None => return false
            }
        }
    }
    while pos <= digits.len(){
        if pos%2!=0{
            result = result + digits[digits.len()-pos];
        }
        else {
           if digits[digits.len()-pos]*2>9{
               result = result + ( (digits[digits.len()-pos]*2) - 9);
           }
            else{
                result=result + (digits[digits.len()-pos]*2);
            }
        }
        pos=pos+1;
    }

    return if result % 10 == 0 {
        true
    } else {
        false
    }
}
