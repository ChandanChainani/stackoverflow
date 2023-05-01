package hashicups

import (
	"context"
	"encoding/json"
	"fmt"
	"net/http"
	"strconv"
	"time"

	"github.com/hashicorp/terraform-plugin-sdk/v2/diag"
	"github.com/hashicorp/terraform-plugin-sdk/v2/helper/schema"
)

func dataSourceProjects() *schema.Resource {
	// return &schema.Resource{
	// 	ReadContext: dataSourceProjectsRead,
	// 	Schema: map[string]*schema.Schema{
	// 		"projects": &schema.Schema{
	// 			Type:     schema.TypeSet,
	// 			Computed: true,
	// 			Elem: &schema.Resource{
	// 				Schema: map[string]*schema.Schema{
	// 					"members": &schema.Schema{
	// 						Type:     schema.TypeList,
	// 						Computed: true,
	// 						Elem: &schema.Schema{
	// 							Type: schema.TypeString,
	// 						},
	// 					},
	// 					"owners": &schema.Schema{
	// 						Type:     schema.TypeList,
	// 						Computed: true,
	// 						Elem: &schema.Schema{
	// 							Type: schema.TypeString,
	// 						},
	// 					},
	// 				},
	// 			},
	// 		},
	// 	},
	// }
	return &schema.Resource{
		ReadContext: dataSourceProjectsRead,
		Schema: map[string]*schema.Schema{
			"members": &schema.Schema{
				Type:     schema.TypeList,
				Elem:     &schema.Schema{Type: schema.TypeString},
				Computed: true,
			},
			"owners": &schema.Schema{
				Type:     schema.TypeList,
				Elem:     &schema.Schema{Type: schema.TypeString},
				Computed: true,
			},
		},
	}
}

func dataSourceProjectsRead(ctx context.Context, d *schema.ResourceData, m interface{}) diag.Diagnostics {
	client := &http.Client{Timeout: 10 * time.Second}

	// Warning or errors can be collected in a slice type
	var diags diag.Diagnostics

	// req, err := http.NewRequest("GET", fmt.Sprintf("%s/projects", "http://localhost:19090"), nil)
	req, err := http.NewRequest("GET", fmt.Sprintf("%s/projects", "http://localhost:8000"), nil)
	if err != nil {
		return diag.FromErr(err)
	}

	r, err := client.Do(req)
	if err != nil {
		return diag.FromErr(err)
	}
	defer r.Body.Close()

	// projects := make(map[string][]interface{}, 0)
	var projects map[string]interface{}
	err = json.NewDecoder(r.Body).Decode(&projects)
	if err != nil {
		return diag.FromErr(err)
	}

	// projectsMap, _ := projects.(map[string][]interface{})
	if err := d.Set("members", projects["members"]); err != nil {
		return diag.FromErr(err)
	}

	if err := d.Set("owners", projects["owners"]); err != nil {
		return diag.FromErr(err)
	}

	// if projectsMap, ok := projects.(map[string][]interface{}); ok {
	//   if err := d.Set("members", projectsMap["members"]); err != nil {
	//     return diag.FromErr(err)
	//   }

	//   if err := d.Set("owners", projectsMap["owners"]); err != nil {
	//     return diag.FromErr(err)
	//   }
	// }

	// always run
	d.SetId(strconv.FormatInt(time.Now().Unix(), 10))

	return diags
}
