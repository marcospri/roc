app "base64"
    packages { base: "platform" }
    imports [base.Task, BytesDecode.{Decoder} ]
    provides [ main ] to base

IO a : Task.Task a []

Decoder a : BytesDecode.Decoder a

main : IO {}
main =
    # when fromBytes [ 0 ] is
    when fromBytes (Str.toBytes "Hello World") is
        Ok str ->
            Task.putLine str

        Err _ ->
            Task.putLine "sadness"





# ------


fromBytes : List U8 -> Result Str BytesDecode.DecodeError
fromBytes = \bytes ->
    BytesDecode.decode  bytes (decodeBase64 (List.len bytes))


decodeBase64 : Nat -> BytesDecode.Decoder Str
decodeBase64 = \width -> BytesDecode.loop loopHelp { remaining: width, string:  "" }

loopHelp : { remaining : Nat, string : Str } -> Decoder (BytesDecode.Step { remaining : Nat, string : Str } Str)
loopHelp = \{ remaining, string } ->
    if remaining >= 3 then
        helper = \x, y, z ->
            a : U32
            a = Num.intCast x
            b : U32
            b = Num.intCast y
            c : U32
            c = Num.intCast z
            combined = Num.bitwiseOr (Num.bitwiseOr (Num.shiftLeftBy 16 a) (Num.shiftLeftBy 8 b)) c
            Loop
                {
                    remaining: remaining - 3,
                    string: Str.concat string (bitsToChars combined 0)
                }

        BytesDecode.map3 helper
            BytesDecode.u8
            BytesDecode.u8
            BytesDecode.u8

    else if remaining == 0 then
        BytesDecode.succeed (Done string)

    else if remaining == 2 then
        helperX = \x, y ->
            a : U32
            a = Num.intCast x
            b : U32
            b = Num.intCast y
            combined = Num.bitwiseOr (Num.shiftLeftBy 16 a) (Num.shiftLeftBy 8 b)
            Done (Str.concat string (bitsToChars combined 1))

        BytesDecode.map2 helperX
            BytesDecode.u8
            BytesDecode.u8
    else
        # remaining = 1
            BytesDecode.u8
                |> BytesDecode.map (\x -> 
                    a : U32
                    a = Num.intCast x
                    Done (Str.concat string (bitsToChars (Num.shiftLeftBy 16 a) 2)))


bitsToChars : U32, Int * -> Str
bitsToChars = \bits, missing ->
    when Str.fromUtf8 (bitsToCharsHelp bits missing) is
        Ok str -> str
        Err _ -> ""

# Mask that can be used to get the lowest 6 bits of a binary number
lowest6BitsMask : Int *
lowest6BitsMask = 63


bitsToCharsHelp : U32, Int * -> List U8
bitsToCharsHelp = \bits, missing ->
    # Performance Notes
    # `String.cons` proved to be the fastest way of combining characters into a string
    # see also https://github.com/danfishgold/base64-bytes/pull/3#discussion_r342321940
    # The input is 24 bits, which we have to partition into 4 6-bit segments. We achieve this by
    # shifting to the right by (a multiple of) 6 to remove unwanted bits on the right, then `Num.bitwiseAnd`
    # with `0b111111` (which is 2^6 - 1 or 63) (so, 6 1s) to remove unwanted bits on the left.
        
    # any 6-bit number is a valid base64 digit, so this is actually safe
    p =
        Num.shiftRightZfBy 18 bits
            |> Num.intCast
            |> unsafeToChar 

    q =
        Num.bitwiseAnd (Num.shiftRightZfBy 12 bits) lowest6BitsMask
            |> Num.intCast
            |> unsafeToChar 

    r =
        Num.bitwiseAnd (Num.shiftRightZfBy 6 bits) lowest6BitsMask
            |> Num.intCast
            |> unsafeToChar 

    s =
        Num.bitwiseAnd bits lowest6BitsMask
            |> Num.intCast
            |> unsafeToChar 

    equals : U8
    equals = 61

    when missing is
        0 -> 
            [ p, q, r, s ]
        1 ->
            [ p, q, r, equals ]
        2 ->
            [ p, q, equals , equals ]
        _ ->
            # unreachable
            []

# Base64 index to character/digit
unsafeToChar : U8 -> U8
unsafeToChar = \n ->
    if n <= 25 then
        # uppercase characters
        65 + n

    else if n <= 51 then
        # lowercase characters
        97 + (n - 26)

    else if n <= 61 then
        # digit characters
        48 + (n - 52)

    else
        # special cases
        when n is
            62 ->
                # '+'
                43

            63 ->
                # '/'
                47

            _ ->
                # anything else is invalid '\u{0000}'
                0
