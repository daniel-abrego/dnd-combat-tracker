import unittest
import dnd.initiative

class test_initiative(unittest.TestCase):
    
    def test_init(self):
        test = dnd.initiative.Initiative()
        self.assertIsNotNone(test)

if __name__=='__main__':
    unittest.main()