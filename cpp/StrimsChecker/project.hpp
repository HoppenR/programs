#ifndef PROJECT_HEADERS
#define PROJECT_HEADERS
#include <curl/curl.h>
#include <ncurses.h>
#include <yajl/yajl_tree.h>

#include <map>
#include <stdexcept>
#include <string>
#include <vector>

struct StreamKeys {
	std::string title;
	std::string channel;
	std::string rustlers;
	std::string viewers;
	std::string service;
	bool hidden;
};
typedef std::vector<StreamKeys> keyvalvec_t;

class Curl {
private:
	CURL* curlhandle;
	std::string JSONdata;

public:
	Curl();
	~Curl();
	void curl_api(const std::string& URL);
	const std::string& getJSON();
};

class MenuWindow {
private:
	WINDOW* Menu;

public:
	MenuWindow();
	~MenuWindow();
	size_t ncurses_select(const keyvalvec_t& choices);
	void print_menu(const keyvalvec_t& choices, const size_t& highlight);
};

class Yajl {
private:
	bool hasparsedinfo = false;
	keyvalvec_t StreamPairVec;
	keyvalvec_t FilteredPairVec;
	yajl_val node;

public:
	Yajl();
	~Yajl();
	void filter_stream_vector();
	void yajl_parse(const std::string& JSONdata, const char* jsonpath[]);
	const keyvalvec_t& getFilteredPairVec();
};

void open_link(const keyvalvec_t& choices, const size_t& choice);
size_t write_memory_callback(void* contents, size_t size, size_t nmemb,
							 void* userp);
#endif // PROJECT_HEADERS
