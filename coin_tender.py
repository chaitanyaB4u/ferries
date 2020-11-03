#1 - Penny - "P"
#5 - Nickel = "N"
#10 - Dime = "Y"
#100 - Dollar = "D"

# 6 = NP 5+1=6
# 15 = YN = 10 + 5 = 15
# DYNP = 100+10+5+1 = 116


def coin_tender(number):
    result_string = ""

    coin_dict = {"D": 100, "Y": 10, "N": 5, "P": 1}

    for denom, value in coin_dict.items():
        while number >= value:
            result_string += denom
            number -= value

    denomination_list = ["D", "Y", "N", "P"]
    input_list = [100,10,5,1]

    for i in range(0, len(denomination_list)):
        while number >= input_list[i]:
            result_string += denomination_list[i]
            number -= input_list[i]


    while number >= 100:
        result_string += "D"
        number -= 100

    while number >= 10:
        result_string += "Y"
        number -= 10

    while number >= 5:
        result_string += "N"
        number -= 5

    while number >= 1:
        result_string += "P"
        number -= 1
    
    return result_string

print(coin_tender(116))