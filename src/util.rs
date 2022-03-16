pub unsafe fn cstr2ruststr<'a>(s: *const u8) -> &'a str {
    let mut slen = 0usize;
    loop {
        if *s.offset(slen as isize) == 0 {
            break;
        }
        slen += 1;
    }
    let strbytes = core::slice::from_raw_parts(s, slen);
    core::str::from_utf8_unchecked(strbytes)
}
