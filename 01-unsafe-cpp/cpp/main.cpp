#include <algorithm>
#include <iostream>
#include <vector>

using namespace std;

int main(void) {
  vector<int> vec = {1, 2, 3, 4, 5, 6, 7, 8, 9, 10};
  for (const auto &value : vec) {
    if (vec.size() > 20) {
      break;
    }
    vec.push_back(value);
  }

  remove_if(vec.begin(), vec.end(), [](const auto &i) { return i % 2 == 0; });

  for (const auto &value : vec) {
    cout << "Value: " + value << endl;
  }
}
