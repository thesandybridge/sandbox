string = "inthecenterofthiscityeastnortheastnearrotesrathauschimesfromtheberlinclockcanbeheardacrossthesquare"
original = "?OBKRUOXOGHULBSOLIFBBWFLRVQQPRNGKSSOTWTQSJQSSEKZZWATJKLUDIAWINFBNYPVTTMZFPKWGDKZXTJCDIGKUHUAUEKCAR"

print(len(string))
print(len(original))

def letter_to_index(letter):
    return ord(letter) - ord('A')

def index_to_letter(index):
    return chr(index + ord('A'))

def vigenere_decrypt(ciphertext, plaintext):
    keyword_indices = []
    for c, p in zip(ciphertext, plaintext):
        K_i = (letter_to_index(c) - letter_to_index(p)) % 26
        keyword_indices.append(K_i)
    keyword = ''.join(index_to_letter(i) for i in keyword_indices)
    return keyword

# Decrypt each part
part1 = vigenere_decrypt("QQPRNGKSS", "NORTHEAST")
part2 = vigenere_decrypt("FLRV", "EAST")
part3 = vigenere_decrypt("MZFPK", "CLOCK")
part4 = vigenere_decrypt("NYPVTT", "BERLIN")

print((part1, part2, part3, part4))

def vigenere_decrypt(ciphertext, keyword):
    keyword_repeated = (keyword * ((len(ciphertext) // len(keyword)) + 1))[:len(ciphertext)]
    plaintext = []

    for c, k in zip(ciphertext, keyword_repeated):
        p_i = (letter_to_index(c) - letter_to_index(k)) % 26
        plaintext.append(index_to_letter(p_i))

    return ''.join(plaintext)

# Full ciphertext
cipher_text = "OBKRUOXOGHULBSOLIFBBWFLRVQQPRNGKSSOTWTQSJQSSEKZZWATJKLUDIAWINFBNYPVTTMZFPKWGDKZXTJCDIGKUHUAUEKCAR"

# Decrypt using KRYPTOS
keyword = "KRYPTOS"
decrypted_text = vigenere_decrypt(cipher_text, keyword)

print(decrypted_text)


"EE VIRTUALLY EEEEEEE INVISIBLE DIGETAL EEE INTERPRET AT IT EE SHADOW EE FORCES EEEEE LUCID EEE MEMORY E T IS YOUR POSITION SOS RQ"
