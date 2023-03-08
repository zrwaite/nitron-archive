# import required libraries
import numpy as np
from numpy.linalg import norm
 
# define two lists or array
A = np.array([0, 2, 8, 3, 1, 4, 3, 5])
B = np.array([7, 8, 0, 1, 0, 0, 0, 2])
C = np.array([4, 3, 0, 1, 0, 0, 0, 1])
D = np.array([0, 0, 3, 0, 6, 1, 3, 1])
 
print("A:", A)
print("B:", B)
print("C:", C)
print("D:", D)
 

def find_similary(vector1, vector2):
    cosine = np.dot(vector1, vector2)/(norm(vector1) * norm(vector2))
    return cosine

def find_best_match(output_emotion, action_options):
    best_match = 0
    best_match_val = 0
    values = []
    for i in range(len(action_options)):
        similarity = find_similary(output_emotion, action_options[i])
        values.append(similarity)
        if  similarity > best_match_val:
            best_match_val = similarity
            best_match = i

    return {"best_match": i, "comparison_values": values}

print(find_best_match(A, [B, C, D]))