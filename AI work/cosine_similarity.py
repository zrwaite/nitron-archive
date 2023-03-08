import numpy as np
from numpy.linalg import norm
 

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

    return {"best_match": best_match, "comparison_values": values}

