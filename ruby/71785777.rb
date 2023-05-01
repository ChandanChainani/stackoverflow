data = {"displayValue"=>"DNAORF004-N9", "isMulti"=>true, "textValue"=>"DNAORF001-N9", "type"=>"entity_link", "value"=>["seq_fdfdf", "seq_9fdfdfdfd"]}
datasequences = [ :displayValue, :textValue ]
data = data.transform_keys(&:to_sym)
datasequences.each do |textValue|
    puts "textValue is #{textValue.inspect}"
    puts "Value: #{data[textValue]}"
end


value = {
	"dnaSequences": [
		{
			"aliases": [],
			"annotations": [],
			"apiURL": "https://url",
			"archiveRecord": nil,
			"authors": [
				{
					"handle": "dsdsd",
					"id": "ent_dsdsd",
					"name": "dsdsd"
				}
			],
			"bases": "",
			"createdAt": "2020-07-14T21:39:26.991794+00:00",
			"creator": {
				"handle": "dsds",
				"id": "ent_dsdsd",
				"name": "dsdd Fdsdsdso"
			},
			"customFields": {},
			"dnaAlignmentIds": [],
			"entityRegistryId": "MOUSE006",
			"fields": {
				"Identical mouses": {
					"displayValue": nil,
					"isMulti": false,
					"textValue": nil,
					"type": "part_link",
					"value": nil
				},
				"Library Constructed By": {
					"displayValue": "dsdsd",
					"isMulti": false,
					"textValue": "dsdsd",
					"type": "dropdown",
					"value": "sfso_dsdsd"
				},
				"Library Construction Date": {
					"displayValue": "2020-06-01",
					"isMulti": false,
					"textValue": "2020-06-01",
					"type": "date",
					"value": "2020-06-01"
				},
				"Library Description": {
					"displayValue": "dsdsdds",
					"isMulti": false,
					"textValue": "dsdsdsd",
					"type": "text",
					"value": "dsdsdsdsd"
				},
				"Library Sample Source": {
					"displayValue": "dsdsds",
					"isMulti": false,
					"textValue": "dsdsdsds",
					"type": "dropdown",
					"value": "sfso_dsdsdsd"
				},
				"ORF": {
					"displayValue": "DNAORF004-N9, DNAORF005-N9, DNAORF008-N9, DNAORF001-N9",
					"isMulti": true,
					"textValue": "DNAORF004-N9, DNAORF005-N9, DNAORF008-N9, DNAORF001-N9",
					"type": "entity_link",
					"value": [
						"seq_aaaaaa",
						"seq_bbbbbb",
						"seq_ccccc",
						"seq_ddddd"
					]
				},
				"Re-Run ORF?": {
					"displayValue": nil,
					"isMulti": false,
					"textValue": nil,
					"type": "dropdown",
					"value": nil
				},
				"Sampling GPS Coordinates": {
					"displayValue": nil,
					"isMulti": false,
					"textValue": nil,
					"type": "text",
					"value": nil
				},
				"Sequencing Approach": {
					"displayValue": "Single Sequence",
					"isMulti": false,
					"textValue": "Single Sequence",
					"type": "dropdown",
					"value": "gfgf"
				},
				"Sequencing Method": {
					"displayValue": "gfgf fgfgfg",
					"isMulti": false,
					"textValue": "gfgf gfgfg",
					"type": "dropdown",
					"value": "sfsogfgfg_irlx6NfZ"
				}
			},
			"folderId": "gfgfg",
			"id": "gfgfgf",
			"isCircular": false,
			"length": 25129,
			"modifiedAt": "2022-04-05T17:06:25.491926+00:00",
			"name": "COPE03-P19",
			"primers": [],
			"registrationOrigin": {
				"originEntryId": nil,
				"registeredAt": "2020-07-14T22:15:09.541243+00:00"
			},
			"registryId": "gfgfgfg",
			"schema": {
				"id": "ps_fdfdfd",
				"name": "mouse"
			},
			"translations": [],
			"url": "hyyps:///COPE/f//edit",
			"webURL": "https://url"
		}
	]
}

# data = value["dnaSequences".to_sym][0]["fields".to_sym]["ORF".to_sym]
datasequences = [ :displayValue, :isMulti, :textValue, :type, :value ]
result = value["dnaSequences".to_sym].map do |v|
  row = {}
  if v.key?(:fields) && v[:fields].key?(:ORF)
    datasequences.map do |key|
      row[key] = v[:fields][:ORF][key.to_sym]
    end
  end
  row
end
puts result

# datasequences.each do |textValue|
    # puts "textValue is #{textValue.inspect}"
    # puts "Value: #{data[textValue]}"
# end
