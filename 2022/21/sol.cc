#include <bits/stdc++.h>

using namespace std;

struct Monkey {
	int64_t result = 0;
	string left;
	string op;
	string right;
};

int64_t  getValue(map<string, Monkey> &graph, string root) {
	string_view op = graph[root].op;
       	if (op == "") {
		return graph[root].result;
	}
	int64_t left = getValue(graph, graph[root].left);
	int64_t right = getValue(graph, graph[root].right);
	if (op == "+") {
		graph[root].result = left + right;
		graph[root].op = "";
	}
	if (op == "-") {
		graph[root].result = left - right;
		graph[root].op = "";
	}
	if (op == "*") {
		graph[root].result = left * right;
		graph[root].op = "";
	}
	if (op == "/") {
		graph[root].result = left / right;
		graph[root].op = "";
	}
	return graph[root].result;
}

int main() {
	string token;
	map<string, Monkey> monkies;
	while (cin >> token) {
		string name = token.substr(0, token.size()-1);
		Monkey& monkey = monkies[name];
		cin >> token;
		if (isdigit(token[0])) {
			monkey.result = stoi(token);
		} else {
			monkey.left = token;
			cin >> monkey.op;
			cin >> monkey.right;
		}
	}

	cout << getValue(monkies, "root") << endl;
	return 0;
}
