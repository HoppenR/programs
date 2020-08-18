const fs = require("fs");
const https = require("https");
const startMonth = 11; //zero based
const startYear = 2013;
const overrustleLog = "Destinygg";
const overrustlePage = "overrustlelogs.net";

function generateTotalMonths() {
	var D = new Date();
	var m = D.getMonth();
	var y = D.getFullYear();
	var monthsToGet = 12 * (y - startYear) + (m - startMonth);
	return monthsToGet;
}

function indexToTime(ix) {
	var month = (ix + startMonth) % 12;
	var year = Math.floor((ix + startMonth) / 12) + startYear;
	return {year: year, month: month};
}

function logTextFromArray(logArray, name) {
	var fileLocation = `${__dirname}/logs/${name}.txt`;
	var logger = fs.createWriteStream(fileLocation);
	let lineswritten = 0;
	for(var i = 0; i < logArray.length; i++) {
		lineswritten += (logArray[i].match(/\n/g) || '').length;
		logger.write(logArray[i]);
	}
	logger.end();
	console.log(`Writing to ${fileLocation}\nType exit or quit to exit program`);
	console.log(`Got ${lineswritten} lines`);
	process.stdout.write("Or enter another name...");
}

function requestLogs(name, monthsToGet, callback) {
	var logArray = [];
	var validMonths = 0;
	var months = [
		"January", "February", "March", "April", "May", "June", "July",
		"August", "September", "October", "November", "December"
	];
	var responsesGotten = 0;
	for(var i = 0; i <= monthsToGet; i++) {
		(function(i) {
			logArray[i] = "";
			var timeObject = indexToTime(i);
			var overrustlePath = `/${overrustleLog}%20chatlog/${months[timeObject.month]}` +
				`%20${timeObject.year}/userlogs/${name}.txt`;
			var options = {
				path: overrustlePath,
				hostname: overrustlePage,
			};
			var request = https.request(options, function(response) {
				console.log(`Status for ${timeObject.year} ${months[timeObject.month]} ${response.statusCode}`);
				response.on("data", function(fd) {
					if(response.statusCode === 200) {
						logArray[i] += fd;
					}
				});
				response.on("end", function() {
					responsesGotten++;
					if(response.statusCode === 200) {
						validMonths++;
					}
					if(responsesGotten > monthsToGet) {
						console.log(`Got ${validMonths} out of ${responsesGotten} possible`);
						callback(logArray, name);
					}
				});
			});
			request.end();
		})(i);
	}
}

function userExists(user, callback) {
	https.get(`https://overrustlelogs.net/api/v1/stalk/${overrustleLog}/${user}.json?limit=1`, function(response) {
		var jsonbody = "";
		response.on("data", function dat(jd) {
			jsonbody += jd;
		});
		response.on("end", function() {
			var jsonobject = JSON.parse(jsonbody);
			if(jsonobject.hasOwnProperty("error")) {
				callback(false);
			} else {
				callback(true);
			}
		});
	}).on('error', function (err) {
		console.error(err);
	});
}

process.stdout.write("Enter username (case sensitive)...");
process.stdin.on("data", function(d) {
	var name = d.toString().trim();
	switch(name) {
		case "help":
			console.log("Type exit or quit to exit program");
			break;
		case "test":
			console.log("insert test case here");
			break;
		case "exit":
		case "quit":
			process.exit();
			break;
		default:
			userExists(name, function(bool) {
				if(bool) {
					var monthsToGet = generateTotalMonths();
					requestLogs(name, monthsToGet, logTextFromArray);
				} else {
					console.log("User not found");
				}
			});
	}
});
