import unittest
from coin_tender import coin_tender as ct

class CoinTenderTest(unittest.TestCase):
    def test_penny(self):
        test = 3
        expected = "PPP"
        self.assertEquals(ct(test), expected)
    
    def test_dollar(self):
        test = 300
        expected = "DDD"
        self.assertEqual(ct(test), expected)

    def test_combination(self):
        test = 20
        expected = "YY"
        self.assertEqual(ct(test), expected)

unittest.main()