pub fn checksum(account: u32) -> bool {
    const BASE : u32 = 10;

    let mut rest = account;
    let mut checksum : u32 = 0;
    for multiplcator in 1..10 {
        let digit = rest % BASE;
        rest = rest / BASE;
        checksum += multiplcator * digit;
    }
    return checksum % 11 == 0;
}