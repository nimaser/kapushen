import random
import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt

def hamming_distance(deck1, deck2):
    score = 0
    for card1, card2 in zip(deck1, deck2):
        if card1 is not card2:
            score += 1
    return score

def riffle_shuffle(deck):
    shuffled_deck = []
    def pick_card(deck1, deck2, shuffled_deck):
        if len(deck1) == 0:
            if len(deck2) == 0:
                shuffled_deck.reverse()
                return shuffled_deck
            else:
                shuffled_deck.append(deck2[-1])
                return pick_card(deck1, deck2[:-1], shuffled_deck)
        elif len(deck2) == 0:
            shuffled_deck.append(deck1[-1])
            return pick_card(deck1[:-1], deck2, shuffled_deck)
        if bool(random.getrandbits(1)):
            shuffled_deck.append(deck1[-1])
            return pick_card(deck1[:-1], deck2, shuffled_deck)
        else:
            shuffled_deck.append(deck2[-1])
            return pick_card(deck1, deck2[:-1], shuffled_deck)
    first_half = deck[:len(deck)//2]
    second_half = deck[len(deck)//2:]
    return pick_card(first_half, second_half, shuffled_deck)

def shuffle_n(deck, n):
    shuffled_deck = deck.copy()
    for _ in range(n):
        shuffled_deck = riffle_shuffle(shuffled_deck)
    return shuffled_deck

ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"]
suits = ["S", "C", "D", "H"]
unshuffled_deck = []

for suit in suits:
    for rank in ranks:
        unshuffled_deck.append(rank + suit)

num_cards = len(unshuffled_deck)

shuffled_deck = unshuffled_deck.copy()

counts = np.zeros((num_cards, num_cards))

num_iterations = 100000

for _ in range(num_iterations):
    shuffled_deck = shuffle_n(unshuffled_deck, 10)
    for col, card in enumerate(shuffled_deck):
        row = unshuffled_deck.index(card)
        counts[row, col] += 1

pluralities = np.zeros(num_cards)

for col in range(num_cards):
    pluralities[col] = max(counts[:, col]) / num_iterations

print(pluralities)

sns.heatmap(counts / num_iterations)
plt.show()
