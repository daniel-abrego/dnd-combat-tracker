import unittest
import dnd.initiative

class test_initiative(unittest.TestCase):

    def test_init(self):
        test = dnd.initiative.Initiative()
        self.assertIsNotNone(test)
    
    def test_add_combatant(self):
        test = dnd.initiative.Initiative()
        test.add_combatant(dnd.combatant.Combatant(),1,1)
        self.assertEquals(0,0)

if __name__=='__main__':
    unittest.main()