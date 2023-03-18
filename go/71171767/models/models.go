package models

import (
	"github.com/google/uuid"
)

type UserDetails struct {
	Name        string    `json:"name"`
	Email       string    `json:"email"`
	Id          uuid.UUID `json:"id" gorm:"unique; type:uuid; column:id; default:uuid_generate_v4(); not_null"`
	OccupantJid string    `json:"occupant_jid"`
	JoinedAt    int64     `json:"joined_at"`
	LeftAt      int64     `json:"left_at"`
}

type RequestData struct {
	EventName    string        `json:"event_name"`
	RoomName     string        `json:"room_name"`
	RoomJID      string        `json:"room_jid" gorm:"primaryKey"`
	StartedAt    int64         `json:"created_at"`
	DestroyedAt  int64         `json:"destroyed_at"`
	UserID       uuid.UUID     `gorm:"unique; not_null"`
	AllOccupants []UserDetails `gorm:"foreignkey:Id;references:UserID" json:"all_occupants"`
}
