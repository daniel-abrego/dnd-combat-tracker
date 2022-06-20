import combatant

class Initiative:
    def __init__(self):
        self.initiative = {}
        self.combatants = []
        self.rounds = 0

    def add_combatant(self, new_combatant: combatant, initiative: int, dex_mod: int):
        if new_combatant not in self.combatants:
            self.combatants.append(new_combatant)
        
        if initiative not in self.initiative:
            self.initiative[initiative] = []
        self.initiative[initiative].append(combatant)

    def print_combatants(self):
        for c in self.combatants:
            print(str(c))

    def print_initiative(self):
        for i in self.initiative:
            print(str(i))

    def print_initiative_combatants(self) -> None:
        print(str(self.initiative_combatants))

def main():
    combat_initiative = Initiative()
    steve = combatant.Combatant('Steve', None)
    steve.change_stats(steve.Stats.DEX, 20)
    alex = combatant.Combatant('Alex', None)
    alex.change_stats(alex.Stats.DEX, 10)
    combat_initiative.add_combatant(steve, 20, 1)
    combat_initiative.add_combatant(alex, 20, 2)
    combat_initiative.print_combatants()
    combat_initiative.print_initiative()
    print('This is the end of main()')
    exit(0)

if __name__=='__main__':
    main()