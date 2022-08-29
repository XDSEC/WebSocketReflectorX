package cache

import (
	"fmt"
	"go.etcd.io/bbolt"
	"time"
	"wsrx/src/config"
)

var Cache *bbolt.DB

func InitCache() error {
	var err error
	Cache, err = bbolt.Open(config.ServerConfig.Cache.CachePath, 0600, &bbolt.Options{Timeout: 1 * time.Second})
	if err != nil {
		return err
	}
	if err := Cache.Update(func(tx *bbolt.Tx) error {
		if tx.Bucket([]byte("Default")) == nil {
			_, err := tx.CreateBucket([]byte("Default"))
			if err != nil {
				return fmt.Errorf("create bucket: %s", err)
			}
		}
		return nil
	}); err != nil {
		return err
	}
	return err
}
