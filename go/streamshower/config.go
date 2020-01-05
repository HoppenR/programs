package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

type ConfigData struct {
	ClientID     string `json:"client_id"`
	ClientSecret string `json:"client_secret"`
	UserName     string `json:"user_name"`
}

func absExeDir() (string, error) {
	name := os.Args[0]
	var err error
	if name[0] == '.' {
		name, err = filepath.Abs(name)
		if err == nil {
			name = filepath.Clean(name)
		}
	} else {
		name, err = exec.LookPath(filepath.Clean(name))
	}
	return filepath.Dir(name), err
}

func (cfg *ConfigData) Load(filename string) error {
	path, err := absExeDir()
	if err != nil {
		return err
	}
	file, err := os.Open(filepath.Join(path, filename))
	if err != nil {
		return err
	}
	defer file.Close()
	dec := json.NewDecoder(file)
	err = dec.Decode(&cfg)
	if err != nil {
		return err
	}
	return nil
}

func (cfg *ConfigData) GetFromUserInput() error {
	rdr := bufio.NewReader(os.Stdin)
	fmt.Print("Please input Client ID: ")
	clientID, err := rdr.ReadString('\n')
	if err != nil {
		fmt.Println()
		return err
	}
	cfg.ClientID = strings.TrimSpace(clientID)
	fmt.Print("Please input Client Secret: ")
	clientSecret, err := rdr.ReadString('\n')
	if err != nil {
		fmt.Println()
		return err
	}
	cfg.ClientSecret = strings.TrimSpace(clientSecret)
	fmt.Print("Please input User Name: ")
	userName, err := rdr.ReadString('\n')
	if err != nil {
		fmt.Println()
		return err
	}
	cfg.UserName = strings.TrimSpace(userName)
	return nil
}

func (cfg *ConfigData) Save(filename string) error {
	path, err := absExeDir()
	if err != nil {
		return err
	}
	file, err := os.Create(filepath.Join(path, filename))
	if err != nil {
		return err
	}
	defer file.Close()
	enc := json.NewEncoder(file)
	enc.SetIndent("", "    ")
	enc.Encode(cfg)
	return nil
}
