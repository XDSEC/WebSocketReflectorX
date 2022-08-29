package utility

import (
	"os"
)

func PathExists(path string) (bool, error) {
	_, err := os.Stat(path)
	if err == nil {
		return true, nil
	}
	if os.IsNotExist(err) {
		return false, nil
	}
	return false, err
}

func CreateDir(v string) error {
	exist, err := PathExists(v)
	if err != nil {
		return err
	}
	if !exist {
		err = os.MkdirAll(v, os.ModePerm)
		if err != nil {
			return err
		}
	}
	return err
}
