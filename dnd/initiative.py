import combatant

class Initiative:
    def __init__(self):
        self.initiative = []
        self.combatants = []
        self.rounds = 0

    def add_combatant(self, new_combatant: combatant, initiative: int, dex_mod: int):
        if initiative not in self.initiative:
            self.inititiave[initiative] = []
        self.initiative[initiative].append(combatant)

    def print_initiative(self):
        for i in initiative:
            for c in i:
                print(str(c))

    def print_initiative_combatants(self) -> None:
        print(str(self.initiative_combatants))

def main():
    combat_initiative = Initiative()
    combat_initiative.add_combatant(1, 'Steve', 1)
    combat_initiative.add_combatant(20, 'Alex', 2)
    combat_initiative.print_initiative()
    exit(0)

if __name__=='__main__':
    main()