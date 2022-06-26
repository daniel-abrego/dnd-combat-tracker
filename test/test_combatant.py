import unittest
import dnd.combatant

class test_combatant(unittest.TestCase):

    def test_init(self):
        test = dnd.combatant.Combatant(name='test', stats=None)
        print(test)
        self.assertIsNotNone(test)
    
    def test_generate_stat(self):
        test = dnd.combatant.Combatant(name='test', stats=None)
        for i in range(20):
            test_stat = test.generate_stat()
            print('test_stat: ' + str(test_stat))
            self.assertIsInstance(test_stat, int)
            self.assertGreaterEqual(test_stat, 3)
            self.assertLessEqual(test_stat, 18)

if __name__=='__main__':
    unittest.main()