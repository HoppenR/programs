#include "./project.hpp"
#define USERNAME "41429754"
#define CLIENTID "vq0unzlra8zlgoi131aihsch2dx9qn"

int main(int argc, char* argv[]) {
	Curl CurlClient;
	MenuWindow Menu;
	Yajl YajlClient;
	const char* followsjsonpath[] = { "follows", NULL };
	const char* livesjsonpath[] = { "streams", NULL };
	const std::string followsurl("https://api.twitch.tv/kraken/users/" USERNAME
								 "/follows/channels?limit=100");
	const std::string livesurl(
		"https://api.twitch.tv/kraken/streams/?channel=");
	size_t selectedstream;
	std::vector<bool> parseopts(OptIndex::LAST_ENUM_NUM_ITEMS, false);
	try {
		if (parse_args(argc, argv, parseopts, selectedstream) == 1)
			return EXIT_FAILURE;
		CurlClient.curl_api(followsurl);
		YajlClient.yajl_parse_follows(CurlClient.getJSON(), followsjsonpath);
		CurlClient.resetjson();
		CurlClient.curl_api(livesurl + YajlClient.get_streams_string());
		YajlClient.reset_streams_string();
		YajlClient.yajl_parse_lives(CurlClient.getJSON(), livesjsonpath);
		YajlClient.filter_stream_vector(parseopts);
		if (parseopts[OptIndex::selectstream]) {
			// figure out if we were given a stream number in optarg before we
			// go to this branch
			open_link(YajlClient.getFilteredPairVec(), selectedstream);
		} else if (parseopts[OptIndex::dmenuselect]) {
			// TODO: dmenu function
			// dmenu_open(YajlClient.getFilteredPairVec());
		} else {
			open_link(YajlClient.getFilteredPairVec(),
					  Menu.ncurses_select(YajlClient.getFilteredPairVec(),
										  parseopts));
		}
	} catch (const std::exception& err) {
		Menu.~MenuWindow();
		fprintf(stderr, "%s\n", err.what());
		return EXIT_FAILURE;
	}
	return EXIT_SUCCESS;
}

Curl::Curl() {
	curlhandle = curl_easy_init();
}

Curl::~Curl() {
	curl_easy_cleanup(curlhandle);
}

void Curl::curl_api(const std::string& URL) {
	CURLcode res;
	curl_slist* headerlist = {};

	headerlist = curl_slist_append(headerlist, "Client-ID: " CLIENTID);
	headerlist = curl_slist_append(headerlist,
								   "Accept: application/vnd.twitchtv.v5+json");
	curl_easy_setopt(curlhandle, CURLOPT_HTTPHEADER, headerlist);
	curl_easy_setopt(curlhandle, CURLOPT_URL, URL.c_str());
	curl_easy_setopt(curlhandle, CURLOPT_USERAGENT, "libcurl-agent/1.0");
	curl_easy_setopt(curlhandle, CURLOPT_WRITEDATA, &JSONdata);
	curl_easy_setopt(curlhandle, CURLOPT_WRITEFUNCTION, write_memory_callback);
	res = curl_easy_perform(curlhandle);
	if (res != CURLE_OK) {
		throw(std::runtime_error(curl_easy_strerror(res)));
	}
	if (JSONdata.size() <= 1) {
		throw(std::runtime_error("CURL Data size is invalid"));
	}
	return;
}

const std::string& Curl::getJSON() {
	return JSONdata;
}

void Curl::resetjson() {
	JSONdata = "";
}

MenuWindow::MenuWindow() {
	std::setlocale(LC_ALL, "en_US.UTF-8");
	initscr();
	Menu = newwin(LINES - 1, COLS, 1, 0);
	noecho();
	cbreak();
	keypad(Menu, true);
	refresh();
}

MenuWindow::~MenuWindow() {
	clear();
	refresh();
	endwin();
}

size_t MenuWindow::ncurses_select(const keyvalvec_t& choices,
								  std::vector<bool> parseopts) {
	bool selected = false;
	int c;
	size_t highlight = 0;

	while (!selected) {
		print_menu(choices, highlight, parseopts);
		c = wgetch(Menu);
		switch (c) {
		case KEY_RESIZE:
			wclear(Menu);
			wrefresh(Menu);
			wresize(Menu, LINES - 1, COLS);
			refresh();
			break;
		case KEY_UP:
		case 107: // key 'k'
			if (highlight == 0) {
				highlight = (choices.size() - 1);
			} else {
				highlight -= 1;
			}
			break;
		case KEY_DOWN:
		case 106: // key 'j'
			if (highlight == (choices.size() - 1)) {
				highlight = 0;
			} else {
				highlight += 1;
			}
			break;
		case 108: // key 'l'
		case 10:  // key 'Enter'
			selected = true;
			break;
		case 113: // key 'q'
			throw(std::runtime_error("Exiting process..."));
			break;
		default:
			break;
		}
	}
	return highlight;
}

void MenuWindow::print_menu(const keyvalvec_t& choices, const size_t& highlight,
							std::vector<bool> parseopts) {
	int y = 1;
	size_t i;
	std::string selectedtitle;

	box(Menu, 0, 0);
	for (auto iter = choices.begin(); iter != choices.end(); iter++) {
		i = unsigned(std::distance(choices.begin(), iter));
		if (i == highlight) {
			wattron(Menu, A_REVERSE);
			selectedtitle = iter->name;
		}
		std::string safeTitle = iter->status;
		safeTitle.erase(std::remove(safeTitle.begin(), safeTitle.end(), '\n'),
						safeTitle.end());
		// TODO: add live-time as an argument option
		// TODO: fix one hour behind on non-daylight savings time
		tzset();
		int timelive_hrs =
			(std::stoi(iter->created_at.substr(11, 2)) - (timezone / 3600)) %
			24;
		int timelive_mns = std::stoi(iter->created_at.substr(14, 2));
		if (timelive_hrs < 0)
			timelive_hrs += 24;
		if (parseopts[OptIndex::showtitle]) {
			mvwprintw(Menu, y, 1,
					  "%-6s%-18s%-19.19s%s%-*.*s%slive since %2.2d:%2.2d",
					  iter->viewers.c_str(), iter->name.c_str(),
					  iter->game.c_str(), iter->game.size() >= 20 ? "▏" : " ",
					  (COLS - 2 - 6 - 18 - 20 - 17),
					  (COLS - 2 - 6 - 18 - 20 - 17), safeTitle.c_str(),
					  safeTitle.size() > unsigned(COLS - 2 - 6 - 18 - 20 - 17)
						  ? "▏"
						  : " ",
					  timelive_hrs, timelive_mns);
		} else {
			mvwprintw(Menu, y, 1, "%-6s%-18s%-20.19s%*s live since %2.2d:%2.2d",
					  iter->viewers.c_str(), iter->name.c_str(),
					  iter->game.c_str(), (COLS - 2 - 6 - 18 - 20 - 17), " ",
					  timelive_hrs, timelive_mns);
		}
		if (i == highlight)
			wattroff(Menu, A_REVERSE);
		++y;
		mvwaddch(Menu, y, 0, ACS_LTEE);			   /* ├ */
		mvwhline(Menu, y, 1, ACS_HLINE, COLS - 2); /*───*/
		mvwaddch(Menu, y, COLS - 1, ACS_RTEE);	   /* ┤ */
		++y;
	}
	wrefresh(Menu);
	mvprintw(0, 1, "Select: %s", selectedtitle.c_str());
	clrtoeol();
	refresh();
	return;
}

Yajl::Yajl() {}
Yajl::~Yajl() {
	if (hasparsedinfo) {
		yajl_tree_free(node);
	}
}

void Yajl::filter_stream_vector(std::vector<bool> parseopts) {
	for (auto iter = StreamPairVec.begin(); iter != StreamPairVec.end();
		 iter++) {
		if (!parseopts[OptIndex::showreruns] &&
			(iter->stream_type == "rerun" ||
			 iter->status.substr(1, 7) == "[RERUN]" ||
			 iter->status.substr(0, 7) == "[Rerun]" ||
			 iter->status.substr(0, 5) == "RERUN"))
			continue;
		if (iter->stream_type == "watch_party" &&
			!parseopts[OptIndex::showvods])
			continue;
		this->FilteredPairVec.push_back(*iter);
	}
}

void Yajl::yajl_parse_follows(const std::string& JSONdata,
							  const char* followsjsonpath[]) {
	char errbuf[1024];
	size_t i;
	size_t j;
	size_t k;
	yajl_val info;

	node = yajl_tree_parse(JSONdata.c_str(), errbuf, sizeof(errbuf));
	if (node == NULL) {
		if (!std::string(errbuf).empty()) {
			throw(std::runtime_error(errbuf));
		} else {
			throw(std::runtime_error("YAJL: unknown error"));
		}
	}
	info = yajl_tree_get(node, followsjsonpath, yajl_t_array);
	// TODO: return early here and strimschecker
	// TODO: find a better way to traverse the path
	// (https://lloyd.github.io/yajl/yajl-2.0.1/yajl__tree_8h.html) ?
	if (info && YAJL_IS_ARRAY(info)) {
		this->hasparsedinfo = true;
		for (i = 0; i < info->u.array.len; ++i) {
			const yajl_val& obj = info->u.array.values[i];
			for (j = 0; j < obj->u.object.len; ++j) {
				if (std::string(obj->u.object.keys[j]) == "channel") {
					const yajl_val& channel = obj->u.object.values[j];
					for (k = 0; k < channel->u.object.len; ++k) {
						if (std::string(channel->u.object.keys[k]) == "_id") {
							streamsstring +=
								YAJL_GET_STRING(channel->u.object.values[k]);
							streamsstring += ",";
						}
					}
				}
			}
		}
	} else {
		throw(std::runtime_error("no such node: followsjsonpath"));
	}
}

void Yajl::yajl_parse_lives(const std::string& JSONdata,
							const char* livesjsonpath[]) {
	char errbuf[1024];
	size_t i;
	size_t j;
	size_t k;
	yajl_val info;

	node = yajl_tree_parse(JSONdata.c_str(), errbuf, sizeof(errbuf));
	if (node == NULL) {
		if (!std::string(errbuf).empty()) {
			throw(std::runtime_error(errbuf));
		} else {
			throw(std::runtime_error("YAJL: unknown error"));
		}
	}
	info = yajl_tree_get(node, livesjsonpath, yajl_t_array);
	// TODO: todos in yajl_parse_follows
	if (info && YAJL_IS_ARRAY(info)) {
		this->hasparsedinfo = true;
		for (i = 0; i < info->u.array.len; ++i) {
			StreamKeys Stream;
			bool valid = true;
			const yajl_val& obj = info->u.array.values[i];
			for (j = 0; j < obj->u.object.len; ++j) {
				const std::string& key(obj->u.object.keys[j]);
				if (key == "viewers") {
					Stream.viewers = YAJL_GET_NUMBER(obj->u.object.values[j]);
				}
				if (key == "game") {
					Stream.game = YAJL_GET_STRING(obj->u.object.values[j]);
				}
				if (key == "stream_type") {
					Stream.stream_type =
						YAJL_GET_STRING(obj->u.object.values[j]);
				}
				if (key == "created_at") {
					Stream.created_at =
						YAJL_GET_STRING(obj->u.object.values[j]);
				}
				if (key == "channel") {
					const yajl_val& channel = obj->u.object.values[j];
					for (k = 0; k < channel->u.object.len; ++k) {
						const std::string& channel_key(
							channel->u.object.keys[k]);
						if (channel_key == "name") {
							Stream.name =
								YAJL_GET_STRING(channel->u.object.values[k]);
						}
						if (channel_key == "status") {
							Stream.status =
								YAJL_GET_STRING(channel->u.object.values[k]);
						}
					}
				}
			}
			if (valid)
				StreamPairVec.push_back(Stream);
		}
	} else {
		throw(std::runtime_error("no such node: livesjsonpath"));
	}
}

const std::string& Yajl::get_streams_string() {
	return streamsstring;
}

const keyvalvec_t& Yajl::getFilteredPairVec() {
	return FilteredPairVec;
}

void Yajl::reset_streams_string() {
	streamsstring = "";
}

void open_link(const keyvalvec_t& choices, const size_t& choice) {
	const char* browser(std::getenv("BROWSER"));
	std::string syscall;

	if (browser == nullptr) {
		throw(std::runtime_error("BROWSER is unset"));
	}
	syscall += browser;
	syscall += ' ';
	syscall += '\"';
	syscall += "https://player.twitch.tv/?channel=";
	syscall += choices[choice].name;
	syscall += "&parent=strims.gg";
	syscall += '\"';
	system(syscall.c_str());
	return;
}

size_t write_memory_callback(void* contents, size_t size, size_t nmemb,
							 void* userp) {
	size_t realsize = (size * nmemb);
	std::string* JSON = static_cast<std::string*>(userp);

	JSON->append(std::string(static_cast<char*>(contents)));
	if (JSON->empty()) {
		throw(
			std::runtime_error("error copying over memory from curl callback"));
	}
	return realsize;
}

int parse_args(int& argc, char* argv[], std::vector<bool>& parseopts,
			   size_t& selectedstream) {
	char c;
	int option_index = 0;
	// TODO: implement long option handling
	// TODO: print relevant error messages on non passed required arguments and
	// other error messages
	static struct option long_options[] = {
		{ "all", no_argument, 0, 0 },		   { "dmenu", no_argument, 0, 0 },
		{ "help", no_argument, 0, 0 },		   { "reruns", no_argument, 0, 0 },
		{ "select", required_argument, 0, 0 }, { "title", no_argument, 0, 0 },
		{ "vods", no_argument, 0, 0 },		   { 0, 0, 0, 0 }
	};
	while (1) {
		c = getopt_long(argc, argv, "-adhrs:tv", long_options, &option_index);
		if (c == -1)
			break;
		switch (c) {
		case 'a':
			parseopts[OptIndex::showreruns] = true;
			parseopts[OptIndex::showvods] = true;
			break;
		case 'd':
			parseopts[OptIndex::dmenuselect] = true;
			break;
		case 'h':
			script_info();
			return EXIT_FAILURE;
			break;
		case 'r':
			parseopts[OptIndex::showreruns] = true;
			break;
		case 's':
			parseopts[OptIndex::selectstream] = true;
			if (optarg && std::isdigit(optarg[0]))
				selectedstream = std::stoul(optarg);
			else
				throw(std::runtime_error("fix parsing"));
			break;
		case 't':
			parseopts[OptIndex::showtitle] = true;
			break;
		case 'v':
			parseopts[OptIndex::showvods] = true;
			break;
		case '?':
			return EXIT_FAILURE;
			break;
		default:
			throw(std::runtime_error("?? getopt returned code: " +
									 std::to_string(c)));
			break;
		}
	}
	return EXIT_SUCCESS;
}

void script_info() {
	// TODO: implement this
}
