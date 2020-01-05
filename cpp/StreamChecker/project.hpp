#ifndef PROJECT_HEADERS
#define PROJECT_HEADERS
#include <curl/curl.h>
#include <ncurses.h>
#include <yajl/yajl_tree.h>

#include <algorithm>
#include <getopt.h>
#include <stdexcept>
#include <string>
#include <time.h>
#include <vector>

struct StreamKeys {
	std::string status;
	std::string name;
	std::string game;
	std::string stream_type;
	std::string viewers;
	std::string created_at;
};
using keyvalvec_t = std::vector<StreamKeys>;

struct OptIndex {
	enum Values : size_t {
		showreruns,
		showvods,
		selectstream,
		showtitle,
		dmenuselect,
		LAST_ENUM_NUM_ITEMS
	};
};

class Curl {
private:
	CURL* curlhandle;
	std::string JSONdata;

public:
	Curl();
	~Curl();
	void curl_api(const std::string& URL);
	const std::string& getJSON();
	void resetjson();
};

class MenuWindow {
private:
	WINDOW* Menu;

public:
	MenuWindow();
	~MenuWindow();
	// TODO parseopts const &reference ?
	size_t ncurses_select(const keyvalvec_t& choices,
						  std::vector<bool> parseopts);
	void print_menu(const keyvalvec_t& choices, const size_t& highlight,
					std::vector<bool> parseopts);
};

class Yajl {
private:
	bool hasparsedinfo = false;
	keyvalvec_t StreamPairVec;
	keyvalvec_t FilteredPairVec;
	yajl_val node;
	std::string streamsstring;

public:
	Yajl();
	~Yajl();
	void filter_stream_vector(std::vector<bool> parseopts);
	void yajl_parse_follows(const std::string& JSONdata,
							const char* jsonpath[]);
	void yajl_parse_lives(const std::string& JSONdata,
						  const char* followsjsonpath[]);
	const keyvalvec_t& getFilteredPairVec();
	const std::string& get_streams_string();
	void reset_streams_string();
};

void open_link(const keyvalvec_t& choices, const size_t& choice);
size_t write_memory_callback(void* contents, size_t size, size_t nmemb,
							 void* userp);
int parse_args(int& argc, char* argv[], std::vector<bool>& parseopts,
			   size_t& selectedstream);
void script_info();
#endif // PROJECT_HEADERS
