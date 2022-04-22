package global

import (
	"go.etcd.io/bbolt"
	"wsrx/server/models"
)

var (
	Config models.Config
	Cache  *bbolt.DB
)
