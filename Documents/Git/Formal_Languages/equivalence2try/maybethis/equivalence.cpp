#include<iostream>
#include<fstream>
#include<vector>
#include<string>
#include<queue>
using namespace std;
struct letter
{
	struct data
	{
		string color = "white";
		int pos;
		int terminal = 0;
		data* ancestor=NULL;
	};
	string l;
	data* link = new data;
};
vector<vector<letter*>> graph1;
vector<vector<letter*>> graph2;
vector<letter::data*> v1;
vector<letter::data*> v2;
queue<letter::data*> q;
vector<string> alphabet;

void dfs(vector<vector<letter*>> &graph, letter::data* u)
{
	u->color = "gray";
	if (u->terminal == 1 && u->ancestor)
		if (u->ancestor->terminal!=1)
			u->ancestor->terminal = 2;
	for (int i = 0; i<graph[u->pos].size(); i++)
	{
		if (graph[u->pos][i]->link->color == "white")
		{
			graph[u->pos][i]->link->ancestor = u;
			dfs(graph, graph[u->pos][i]->link);
		}
	}
	if (u->terminal > 0 && u->ancestor != NULL)
		if (u->terminal != 1)
			u->ancestor->terminal = 2;
	u->color = "black";
}

bool devil1(letter::data* u)
{
	bool check = true;
	for (int i = 0; i < graph1[u->pos].size(); i++)
	{
		if (graph1[u->pos][i]->link->pos == u->pos && u->terminal == 0)
			check &= true;
		else
			return false;
	}
	return check;
}
bool devil2(letter::data* u)
{
	bool check = true;
	for (int i = 0; i < graph2[u->pos].size(); i++)
	{
		if (graph2[u->pos][i]->link->pos == u->pos && u->terminal == 0)
			check &= true;
		else
			return false;
	}
	return check;
}
bool bfs(letter::data* u1, letter::data* u2)
{
	q.push(u1);
	q.push(u2);
	while (!q.empty())
	{
		letter::data* t1 = q.front();
		q.pop();
		letter::data* t2 = q.front();
		q.pop();
		if (((t1->terminal == 0 || t1->terminal == 2) && t2->terminal == 1) || ((t2->terminal == 0 || t2->terminal == 2) && t1->terminal == 1))
			return false;
		t1->color = "gray";
		t2->color = "gray";
		for (int i = 0; i < alphabet.size(); i++)
		{
			int ind1 = -1, ind2 = -1;
			for (int j = 0; j < graph1[t1->pos].size(); j++)
				if (graph1[t1->pos][j]->l == alphabet[i])
					ind1 = j;
			for (int j = 0; j < graph2[t2->pos].size(); j++)
				if (graph2[t2->pos][j]->l == alphabet[i])
					ind2 = j;
			if ((ind1 == -1 && ind2 != -1) || (ind1 != -1 && ind2 == -1))
			{
				if ((devil1(t1) && devil2(t2)))
					;
				else
					if ((t1->terminal == 2 && t2->terminal == 0) || (t2->terminal == 2 && t1->terminal == 0))
						return false;
			}
			if (ind1 != -1 && ind2 != -1 && (graph1[t1->pos][ind1]->link->color == "white" || graph2[t2->pos][ind2]->link->color=="white"))
			{
				q.push(graph1[t1->pos][ind1]->link);
				q.push(graph2[t2->pos][ind2]->link);
			}
		}
	}
	return true;
}

int main()
{
	ifstream fin("equivalence.in");
	ofstream fout("equivalence.out");
	int n, m, k;
	cin >> n >> m >> k;
	graph1.resize(n);
	v1.resize(n);
	for (int i = 0; i < n; i++)
	{
		letter::data* u = new letter::data;
		u->pos = i;
		v1[i] = u;
	}
	for (int i = 0; i < k; i++)
	{
		int term;
		cin >> term;
		v1[term - 1]->terminal = 1;
	}
	for (int i = 0; i < m; i++)
	{
		int a, b;
		string s;
		cin >> a >> b >> s;
		bool check = true;
		for (int i = 0; i < alphabet.size(); i++)
			if (s == alphabet[i])
				check = false;
		if (check)
			alphabet.push_back(s);
		letter* u = new letter;
		u->l = s;
		u->link = v1[b - 1];
		graph1[a - 1].push_back(u);
	}

	cin >> n >> m >> k;
	graph2.resize(n);
	v2.resize(n);
	for (int i = 0; i < n; i++)
	{
		letter::data* u = new letter::data;
		u->pos = i;
		v2[i] = u;
	}
	for (int i = 0; i < k; i++)
	{
		int term;
		cin >> term;
		v2[term - 1]->terminal = 1;
	}
	for (int i = 0; i < m; i++)
	{
		int a, b;
		string s;
		cin >> a >> b >> s;
		bool check = true;
		for (int i = 0; i < alphabet.size(); i++)
			if (s == alphabet[i])
				check = false;
		if (check)
			alphabet.push_back(s);
		letter* u = new letter;
		u->l = s;
		u->link = v2[b - 1];
		graph2[a - 1].push_back(u);
	}
	dfs(graph1, v1[0]);
	dfs(graph2, v2[0]);
	for (int i = 0; i < v1.size(); i++)
		v1[i]->color = "white";
	for (int i = 0; i < v2.size(); i++)
		v2[i]->color = "white";
	if (bfs(v1[0], v2[0]))
		cout << "YES";
	else
		cout << "NO";
	system("pause");
	return 0;
}