

def getAmas(matrix, x, y):
    target_num = matrix[y][x]
    rows = len(matrix)
    cols = len(matrix[0])
    visited = [[False] * cols for _ in range(rows)]
    
    def dfs(x, y):
        if x < 0 or x >= cols or y < 0 or y >= rows or visited[y][x] or matrix[y][x] != target_num:
            return []
        visited[y][x] = True
        coordinates = [(x, y)]
        for dx, dy in [(1, 0), (-1, 0), (0, 1), (0, -1)]:
            next_x, next_y = x + dx, y + dy
            coordinates += dfs(next_x, next_y)
        return coordinates

    return dfs(x, y)

def findHoles(matrix: list) -> int:
    
    formes = []
    already_visited = []

    for y, row in enumerate(matrix):
        for x, elem in enumerate(row):
        
            # Nouvelle pièce à ajouter à la collection
            if elem == 0 and (x, y) not in already_visited:
                w = getAmas(matrix, x, y)  # Forme W enregistrée.
                # On voudrait éviter de refaire un traitement pour une case
                # qui appartient à W, parce que ça ferait exactement la même forme.
                for coord in w:
                    already_visited.append(coord)
                formes.append(w)
    
    return (len(formes) - 1)



matrice = [[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], [7, 6, 6, 0, 0, 0, 0, 0, 7, 7, 7, 7], [7, 6, 6, 0, 1, 0, 7, 0, 2, 0, 0, 0], [7, 0, 1, 1, 1, 4, 7, 0, 2, 2, 2, 0], [7, 4, 4, 0, 4, 4, 7, 0, 5, 0, 6, 6], [7, 0, 4, 4, 4, 0, 7, 5, 5, 5, 6, 6]]

print(findHoles(matrice))