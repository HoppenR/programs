#include "./project.hpp"
#define RUSTLERS_MIN 9

// TODO: change to a cpp JSON library
// TODO: somehow add filtering to the top of the window which would filter out
// non matching entries
// TODO: handle multi byte characters better
// TODO: fork firefox to the background
// TODO: add this-> where it makes sense for readability inside class functions

int main(void) {
	Curl CurlClient;
	MenuWindow Menu;
	Yajl YajlClient;
	const char* jsonpath[] = { "stream_list", NULL };
	const std::string URL("https://strims.gg/api");
	size_t choice;
	try {
		CurlClient.curl_api(URL);
		YajlClient.yajl_parse(CurlClient.getJSON(), jsonpath);
		YajlClient.filter_stream_vector();
		choice = Menu.ncurses_select(YajlClient.getFilteredPairVec());
		open_link(YajlClient.getFilteredPairVec(), choice);
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

MenuWindow::MenuWindow() {
	setlocale(LC_CTYPE, "");
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

size_t MenuWindow::ncurses_select(const keyvalvec_t& choices) {
	bool selected = false;
	int c;
	size_t highlight = 0;

	while (!selected) {
		print_menu(choices, highlight);
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

void MenuWindow::print_menu(const keyvalvec_t& choices,
							const size_t& highlight) {
	int y = 1;
	size_t i;
	std::string selectedtitle;

	box(Menu, 0, 0);
	for (auto iter = choices.begin(); iter != choices.end(); iter++) {
		i = unsigned(std::distance(choices.begin(), iter));
		mvwprintw(Menu, y, 2, "title   : %.*s", COLS - 13, iter->title.c_str());
		++y;
		if (iter->title.length() > static_cast<size_t>(COLS - 13)) {
			size_t printed = static_cast<size_t>(COLS - 13);
			while (printed < iter->title.length()) {
				mvwprintw(Menu, y, 2, "          %.*s", COLS - 13,
						  iter->title.substr(printed).c_str());
				printed += static_cast<size_t>(COLS - 13);
				++y;
			}
		}
		if (i == highlight) {
			wattron(Menu, A_REVERSE);
			mvwprintw(Menu, y, 2, "channel : %s", iter->channel.c_str());
			wattroff(Menu, A_REVERSE);
			selectedtitle = iter->channel;
		} else {
			mvwprintw(Menu, y, 2, "channel : %s", iter->channel.c_str());
		}
		++y;
		mvwprintw(Menu, y, 2, "rustlers: %s", iter->rustlers.c_str());
		++y;
		mvwprintw(Menu, y, 2, "viewers : %s", iter->viewers.c_str());
		++y;
		mvwprintw(Menu, y, 2, "service : %s", iter->service.c_str());
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

void Yajl::filter_stream_vector() {
	for (auto iter = StreamPairVec.begin(); iter != StreamPairVec.end();
		 iter++) {
		if (std::stoi(iter->rustlers) < RUSTLERS_MIN)
			continue;
		if (iter->hidden)
			continue;
		this->FilteredPairVec.push_back(*iter);
	}
}

void Yajl::yajl_parse(const std::string& JSONdata, const char* jsonpath[]) {
	char errbuf[1024];
	size_t i;
	size_t j;
	yajl_val info;

	node = yajl_tree_parse(JSONdata.c_str(), errbuf, sizeof(errbuf));
	if (node == NULL) {
		if (!std::string(errbuf).empty()) {
			throw(std::runtime_error(errbuf));
		} else {
			throw(std::runtime_error("YAJL: unknown error"));
		}
	}
	info = yajl_tree_get(node, jsonpath, yajl_t_array);
	if (info && YAJL_IS_ARRAY(info)) {
		this->hasparsedinfo = true;
		for (i = 0; i < (info->u.array.len); ++i) {
			StreamKeys Stream;
			const yajl_val& obj = info->u.array.values[i];
			for (j = 0; j < (obj->u.object.len); ++j) {
				const std::string& key(obj->u.object.keys[j]);
				if (key == "hidden") {
					Stream.hidden = YAJL_IS_TRUE(obj->u.object.values[j]);
				}
				if (key == "title") {
					Stream.title = YAJL_GET_STRING(obj->u.object.values[j]);
				} else if (key == "channel") {
					Stream.channel = YAJL_GET_STRING(obj->u.object.values[j]);
				} else if (key == "rustlers") {
					Stream.rustlers = YAJL_GET_NUMBER(obj->u.object.values[j]);
				} else if (key == "viewers") {
					Stream.viewers = YAJL_GET_NUMBER(obj->u.object.values[j]);
				} else if (key == "service") {
					Stream.service = YAJL_GET_STRING(obj->u.object.values[j]);
				}
			}
			StreamPairVec.push_back(Stream);
		}
	} else {
		throw(std::runtime_error(std::string("no such node: ") + jsonpath[0]));
	}
	return;
}

const keyvalvec_t& Yajl::getFilteredPairVec() {
	return FilteredPairVec;
}

void open_link(const keyvalvec_t& choices, const size_t& choice) {
	const char* browser(std::getenv("BROWSER"));
	// TODO: fix the query strings
	const std::map<std::string, std::string> platformMap = {
		{ "angelthump", "https://player.angelthump.com/?channel=" },
		{ "facebook", "https://www.facebook.com/video/embed" },
		{ "mixer", "https://mixer.com/embed/player/" },
		{ "smashcast", "https://www.smashcast.tv/embed/" },
		{ "twitch", "https://strims.gg/twitch/" },
		{ "twitch-vod",
		  R"(https://player.twitch.tv/?parent=strims.gg\&video=v)" },
		{ "ustream", "https://www.ustream.tv/embed/" },
		{ "vaughn", "https://vaughnlive.tv/embed/video/" },
		{ "youtube", "https://www.youtube.com/embed/" },
		{ "youtube-playlist", "https://www.youtube.com/embed/videoseries" },
		{ "m3u8", "https://strims.gg/m3u8/" },
	};
	std::string syscall;

	if (browser == nullptr) {
		throw(std::runtime_error("BROWSER is unset"));
	}
	syscall += browser;
	syscall += " ";
	syscall += platformMap.at(choices[choice].service);
	syscall += choices[choice].channel;
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
