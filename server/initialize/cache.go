package initialize

import (
	"fmt"
	"time"
	"wsrx/server/global"

	"go.etcd.io/bbolt"
)

func InitCache() error {
	var err error
	global.Cache, err = bbolt.Open(global.Config.Cache.Path, 0600, &bbolt.Options{Timeout: 1 * time.Second})
	if err := global.Cache.Update(func(tx *bbolt.Tx) error {
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
