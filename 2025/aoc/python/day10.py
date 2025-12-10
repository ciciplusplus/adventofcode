# pipx run day10.py

# /// script
# dependencies = ["pulp"]
# ///

from pulp import *

# n = 6
# model = LpProblem(sense=LpMinimize)
# variables = [LpVariable(f"a{i}", 0, None, LpInteger) for i in range(n)]
# model += lpDot([1, 1, 0, 1, 0, 0], variables) == 7
# model += lpDot([0, 0, 1, 1, 1, 0], variables) == 4
# model += lpDot([0, 1, 0, 0, 0, 1], variables) == 5
# model += lpDot([0, 0, 0, 0, 1, 1], variables) == 3
# model += lpDot([1, 1, 1, 1, 1, 1], variables)
# status = model.solve()

f = open("../src/day10-part1.txt")

jolts = []
toggles = []

for line in f:
	line = line.strip()

	_, rest = line.split("] (", 1)

	toggles_str, jolt_str = rest.split(") {", 1)

	jolt_data = jolt_str[:-1]

	jolt = [int(x.strip()) for x in jolt_data.split(',')]
	jolts.append(jolt)

	toggle = []
	for toggle_str in toggles_str.split(") ("):
	    toggle_positions = [int(x.strip()) for x in toggle_str.split(',')]
	    toggle.append(toggle_positions)

	toggles.append(toggle)

sum = 0

for idx in range(len(jolts)):
	jolt = jolts[idx]
	toggle = toggles[idx]

	# print(jolts, toggles)

	n = len(toggle)
	m = len(jolt)
	mat = [ [0] * n for _ in range(m) ]

	for i, val in enumerate(toggle):
		for j in val:
			mat[j][i] = 1

	model = LpProblem(sense=LpMinimize)
	variables = [LpVariable(f"a{i}", 0, None, LpInteger) for i in range(n)]
	for j in range(m):
		model += lpDot(mat[j], variables) == jolt[j]

	model += lpDot([1] * n, variables)

	status = model.solve(PULP_CBC_CMD(msg=False))

	sum += int(model.objective.value())

print(sum)