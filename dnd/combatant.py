from enum import Enum
from random import randint

class Combatant:
    class Stats(Enum):
        CON = 0
        DEX = 1
        STR = 2
        INT = 3
        WIS = 4
        CHA = 5
    
    name = ''
    stats = {}
    initiative = 0
    
    def __init__(self, name, stats):
        self.name = name
        if stats:
            self.stats = stats
        else:
            self.stats = {
                self.Stats.CON: self.generate_stat(),
                self.Stats.STR: self.generate_stat(),
                self.Stats.DEX: self.generate_stat(),
                self.Stats.INT: self.generate_stat(),
                self.Stats.WIS: self.generate_stat(),
                self.Stats.CHA: self.generate_stat()
        }
        self.initiative = self.get_initiative()

    def get_initiative(self):
        return self.initiative

    def get_initiative_mod(self):
        return (self.get_initiative() - 10) // 2
    
    def roll_initiative(self):
        self.initiative = randint(1,20) + self.get_initiative_mod()
        return self.initiative

    def change_stats(self, stat, new_stat_val):
        
        assert(new_stat_val <= 20 and new_stat_val >= 1)


    def validate_stat(self, stat):
        return stat <= 20 and stat >= 1

    def generate_stat(self):
        dice_rolls = []
        for i in range(4):
            dice_rolls.append(randint(1,6))
        lowest_roll = 7
        for i in range(len(dice_rolls)):
            if dice_rolls[i] < lowest_roll:
                lowest_roll = dice_rolls[i]
        dice_rolls.remove(lowest_roll)
        assert(True)
        return sum(dice_rolls)
    
    def __repr__(self):
        return "Combatant()"

    def __str__(self):
        return str(self.name) + '(' + str(self.get_initiative_mod()) + ')'
    
if __name__=='__main__':
    test = Combatant(name='test')
    print(test.get_initiative())
    print('get_initiative: ' + str(test.get_initiative()))
    print('roll_initiative: ' + str(test.roll_initiative()))
    print('get_initiative: ' + str(test.get_initiative()))