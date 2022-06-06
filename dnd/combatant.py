from random import randint

class Combatant:
    name = ''
    stats = {}
    initiative = 0
    initiative_mod = 0

    def __init__(self, name):
        self.name = name
        self.stats = {
            'con': self.generate_stat(),
            'str': self.generate_stat(),
            'dex': self.generate_stat(),
            'int': self.generate_stat(),
            'wis': self.generate_stat(),
            'cha': self.generate_stat()
        }
        self.initiative = self.get_initiative()
        self.initiative_mod = (self.stats['dex'] - 10) // 2

    def get_initiative(self):
        return self.initiative
    
    def roll_initiative(self):
        self.initiative = randint(1,20) + self.initiative_mod
        return self.initiative

    def change_stats(self, con, str, dex, int, wis, cha):
        if con and self.validate_stat(con):
            self.stats['con'] = con
        if str and self.validate_stat(str):
            self.stats['str'] = str
        if dex and self.validate_stat(dex):
            self.stats['dex'] = dex
        if int and self.validate_stat(int):
            self.stats['int'] = int
        if wis and self.validate_stat(wis):
            self.stats['wis'] = wis

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
    
if __name__=='__main__':
    test = Combatant(name='test')
    print(test.get_initiative())
    print('get_initiative: ' + str(test.get_initiative()))
    print('roll_initiative: ' + str(test.roll_initiative()))
    print('get_initiative: ' + str(test.get_initiative()))