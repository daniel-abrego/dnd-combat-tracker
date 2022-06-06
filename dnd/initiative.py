import combatant

class Initiative:
    initiative = []

    def __init__(self):
        self.initiative = []
    
    def add_combatant(self, name, initiative, dex_mod) -> None:
        self.initiative[name] = {'init': initiative, 'dex_mod': dex_mod}