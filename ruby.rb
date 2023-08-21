module Converter_WGS84
	# WGS84 - documentation in pdf: /tes/docs/ts_123032v150000p_WGS84.pdf

	# ------------ ENCODE ------------  
	def self.to_hex(decimal_value)
		decimal_value.to_i.to_s(16).upcase
	end

	def self.to_twos_complement(str)
		n = str.length

		i = n - 1
		while i >= 0
			break if str[i] == '1'
			i -= 1
		end

		if i == -1
			return '1' + str
		end

		k = i - 1
		while k >= 0
			# Just flip the values
			if str[k] == '1'
				str[k] = '0'
			else
				str[k] = '1'
			end
				k -= 1
		end

		return str
	end
	
	def self.setLatitude(latitude)
		encoded = []
		latitude = latitude.to_f
		latitude = 90 if latitude > 90
		latitude = -90 if latitude < -90
	
		sign_bit = latitude >= 0 ? 0 : 1
		latitude = latitude.abs
	
		coded_latitude = ((2**23 * latitude) / 90).to_i
	
		encoded_value = (sign_bit << 23) | coded_latitude
		encoded_value = to_hex(encoded_value).rjust(6, '0')

		(0...encoded_value.length).step(2) do |i|
			encoded.append('%#x' % encoded_value[i, 2].to_i(16))
		end

		return encoded
	end

	def self.setLongitude(longitude)
		encoded = []
		longitude = longitude.to_f

		coded_longitude = ((2**24 * (longitude)) / 360).to_i
		
		if coded_longitude < 0
			coded_longitude *= -1
		end

		coded_longitude = coded_longitude.to_s(2).rjust(24, '0')

		if longitude < 0
			coded_longitude = to_twos_complement(coded_longitude)
		end

		coded_longitude = coded_longitude.scan(/.{4}/).map { |group| group.to_i(2).to_s(16).upcase }.join
		encoded_value = coded_longitude.rjust(6, '0')

		(0...encoded_value.length).step(2) do |i|
			encoded.append('%#x' % encoded_value[i, 2].to_i(16))
		end

		return encoded
	end

	def self.setInner_r(param)
		if param == nil 
			return nil 
		end

		encoded = []
		inner_r = to_hex(((2 * param) - 1) / 10)

		if inner_r.length <= 2
			inner_r = "00" + inner_r
		elsif inner_r.length != 4
			inner_r = "0" + inner_r
		end

		(0...inner_r.length).step(2) do |i|
			encoded.append('%#x' % inner_r[i, 2].to_i(16))
		end

		return encoded
	end

	def self.setUncertainty_r(param)
		if param == nil 
			return nil 
		end

		uncertainty_r = to_hex(Math.log(((param / 10) + 1), 1.1))

		if uncertainty_r.length != 2
			uncertainty_r = "0" + uncertainty_r
		end

		return '%#x' % uncertainty_r.to_i(16)
	end

	def self.setOffset_a(param)
		if param == nil 
			return nil 
		end

		offset_a = to_hex(((2 * param) - 1) / 4)

		if offset_a.length != 2
			offset_a = "0" + offset_a
		end

		return '%#x' % offset_a.to_i(16)
	end

	def self.setIncluded_a(param)
		if param == nil 
			return nil 
		end

		included_a = to_hex(((2 * param) - 1) / 4)

		if included_a.length != 2
			included_a = "0" + included_a
		end

		return '%#x' % included_a.to_i(16)
	end

	def self.setConfidence(param)
		if param == nil 
			return nil 
		end

		confidence = to_hex(param)

		if confidence.length != 2
			confidence = "0" + confidence
		end

		return '%#x' % confidence.to_i(16)
	end

	def self.encode(
		latitude:,
		longitude:,
		innerRadiuse: nil,
		uncertaintyRadiuse: nil,
		offsetAngle: nil,
		includedAngle: nil,
		confidence: nil
	    )

		encoded = []
		encoded.append(*setLatitude(latitude))
		encoded.append(*setLongitude(longitude))

		unless innerRadiuse.nil? || uncertaintyRadiuse.nil? || offsetAngle.nil? || includedAngle.nil? || confidence.nil?
			encoded.append(*setInner_r(innerRadiuse))
			encoded.append(setUncertainty_r(uncertaintyRadiuse))
			encoded.append(setOffset_a(offsetAngle))
			encoded.append(setIncluded_a(includedAngle))
			encoded.append(setConfidence(confidence))
		end

		encoded = encoded.compact

		if encoded.length == 6
			encoded.unshift('%#x' % "00".to_i(16))
		elsif encoded.length == 12
			encoded.unshift('%#x' % "A0".to_i(16))
		else
			encoded = []
		end

		encoded = encoded.compact.map { |item| item.to_i(16) }

		return encoded
	end

	# ------------ DECODE ------------ 

	def self.hex_to_binary(hex_value)
        hex_value.to_i(16).to_s(2).to_i(2)
    end

	def self.getLatitude(param)
		encoded_value = hex_to_binary(param)
		sign_bit = encoded_value >> 23
		coded_latitude = encoded_value & ((1 << 23) - 1)
	  
		latitude = ((90 * coded_latitude.to_f) / (2**23))
		latitude = -latitude if sign_bit == 1

		return latitude
	end

	def self.getLongitude(longitude)
		encoded_value = longitude.rjust(6, '0')

		decoded_longitude = encoded_value.scan(/../).map { |group| group.to_i(16).to_s(2).rjust(8, '0') }.join

		sign = 1

		if decoded_longitude[0] == '1'
		  decoded_longitude = from_twos_complement(decoded_longitude)
		  sign = -1
		end
	  
		decoded_longitude = decoded_longitude.to_i(2)
	  
		longitude = (decoded_longitude * 360) / (2**24).to_f * sign
	  
		return longitude
	end

	def self.from_twos_complement(bin_str)
		flipped = bin_str.tr('01', '10')
		result = (flipped.to_i(2) + 1).to_s(2)
		
		return result.rjust(bin_str.length, '0')
	end

	def self.getInner_r(param)
		inner_r = (param).to_i(16)
		inner_r = (10 * (inner_r + 1)) / 2.to_f

		return inner_r
	end

	def self.getUncertainty_r(param)
		uncertainty_r = (param).to_i(16)
		uncertainty_r = (10 * ((1.1)**(uncertainty_r.to_f)) - 1)

		return uncertainty_r
	end
	
	def self.getOffset_a(param)
		offset_a = (param).to_i(16)
		offset_a = (4 * (offset_a + 1)) / 2.to_f

		return offset_a
	end

	def self.getIncluded_a(param)
		included_a = (param).to_i(16)
		included_a = (4 * (included_a + 1)) / 2.to_f

		return included_a
	end

	def self.getConfidence(param)
		confidence = (param).to_i(16)

		return confidence
	end

	def self.decode(wgs84)
		array = wgs84.split(" ")
		values = []

		values.append(array[0])
		values.append(getLatitude((array[1] + array[2] + array[3])))
		values.append(getLongitude((array[4] + array[5] + array[6])))

		if array[0].downcase == "a0"
			values.append(getInner_r((array[7] + array[8])))
			values.append(getUncertainty_r((array[9])))
			values.append(getOffset_a((array[10])))
			values.append(getIncluded_a((array[11])))
			values.append(getConfidence((array[12])))
		end

		return values
	end
end


module Encoder_DMS
	# DMS - Degrees Minutes Seconds coordinates format

	def self.decimal_dms(coordinate)
		coordinate = coordinate.abs
		coordinate_whole = coordinate.to_i
		coordinate_decimal = coordinate - coordinate_whole
		minutes = coordinate_decimal * 60
		minutes_whole = minutes.to_i
		minutes_decimal = minutes - minutes_whole
		seconds = (minutes_decimal * 60.0).to_f.round(6)

		return {
			degrees: coordinate_whole,
			minutes: minutes_whole,
			seconds: seconds
		}
	end

	def self.setLatitude(latitude)
		coordinate = latitude.to_f
		coordinate_direction = coordinate > 0 ? "N" : "S"

		hash_dms = decimal_dms(coordinate)
		hash_dms[:direction] = coordinate_direction

		return hash_dms
	end

	def self.setLongitude(longitude)
		coordinate = longitude.to_f
		coordinate_direction = coordinate > 0 ? "E" : "W"
		
		hash_dms = decimal_dms(coordinate)
		hash_dms[:direction] = coordinate_direction

		return hash_dms
	end

	def self.run(latitude:, longitude:, azimuth: nil)
		dms_latitude = setLatitude(latitude)
		dms_longitude = setLongitude(longitude)

		dms = {
			latitude: dms_latitude,
			longitude: dms_longitude
		}
		dms[:azimuth] = azimuth.round.to_s if azimuth

		return dms
	end
end


class Coordinates
	attr_writer :latitude, :longitude, :inner_radius, :uncertainty_radius, :offset_angle, :included_angle, :confidence
	attr_writer :wgs84, :dms_tim, :dms_wind
	attr_writer :format

	def initialize
		@latitude = nil
		@longitude = nil
		@inner_radius = nil
		@uncertainty_radius = nil
		@offset_angle = nil
		@included_angle = nil
		@confidence = nil
		@format = nil

		@wgs84 = nil
		@dms_tim = nil
		@dms_wind = nil

		@azimuth = nil
	end

	def encode
		encoded = nil

		@azimuth = @offset_angle.nil? ? nil : (@offset_angle.to_f + (@included_angle.to_f/2))

		case @format
		when 'wgs84'
			encoded = encode_wgs84() 	# [ array of bytes in int format ]
		when 'dms_wind'
			encoded = encode_dms_wind() # { latitude: String, longitude: String, azimuth: String }
		when 'dms_tim'
			encoded = encode_dms_tim() 	# { latitude: String, longitude: String, irradiation: String }
		end

		encoded
	end

	def decode
		decoded = nil

		if @wgs84 != nil
			decoded = Converter_WGS84.decode(@wgs84)
		end

		decoded
	end

	private

	def encode_wgs84
		encoded = []

		unless @latitude.nil? || @longitude.nil?
			encoded = Converter_WGS84.encode(
				latitude: @latitude,
				longitude: @longitude,
				innerRadiuse: @inner_radius,
				uncertaintyRadiuse: @uncertainty_radius,
				offsetAngle: @offset_angle,
				includedAngle: @included_angle,
				confidence: @confidence
			)
		end

		return encoded 
	end

	def encode_dms_wind
		encoded = {}

		unless @latitude.nil? || @longitude.nil?
			dms_hash = Encoder_DMS.run(latitude: @latitude, longitude: @longitude, azimuth: @azimuth)

			dms_latitude = dms_hash[:latitude]
			dms_longitude = dms_hash[:longitude]

			dms_latitude[:seconds] = dms_latitude[:seconds].round(3)
			dms_longitude[:seconds] = dms_longitude[:seconds].round(3)
			dms_longitude[:degrees] = dms_longitude[:degrees].to_s.rjust(3, '0')
			

			dms_latitude = "#{dms_latitude[:direction]}#{dms_latitude[:degrees].rjust(2, '0')}#{dms_latitude[:minutes].rjust(2, '0')}#{dms_latitude[:seconds]}"
			dms_longitude = "#{dms_longitude[:direction]}#{dms_longitude[:degrees].rjust(2, '0')}#{dms_longitude[:minutes].rjust(2, '0')}#{dms_longitude[:seconds]}"

			encoded = {
				latitude: dms_latitude,
				longitude: dms_longitude
			}
			encoded[:azimuth] = dms_hash[:azimuth] if dms_hash[:azimuth]
		end

		return encoded 
	end

	def encode_dms_tim
		encoded = {}

		unless @latitude.nil? || @longitude.nil?
			dms_hash = Encoder_DMS.run(latitude: @latitude, longitude: @longitude, azimuth: @azimuth)

			dms_latitude = dms_hash[:latitude]
			dms_longitude = dms_hash[:longitude]

			dms_latitude[:minutes] = dms_latitude[:minutes].to_s.rjust(2, '0')
			dms_latitude[:seconds] = dms_latitude[:seconds].round(2)
			dms_latitude[:direction] = (dms_latitude[:direction] == "N") ? "" : " #{dms_latitude[:direction]}"

			dms_longitude[:minutes] = dms_longitude[:minutes].to_s.rjust(2, '0')
			dms_longitude[:seconds] = dms_longitude[:seconds].round(2)
			

			dms_latitude = "#{dms_latitude[:degrees]} #{dms_latitude[:minutes].rjust(2, '0')} #{dms_latitude[:seconds].rjust(2, '0')}#{dms_latitude[:direction]}"
			dms_longitude = "#{dms_longitude[:degrees]} #{dms_longitude[:minutes].rjust(2, '0')} #{dms_longitude[:seconds].rjust(2, '0')} #{dms_longitude[:direction]}"

			encoded = {
				latitude: dms_latitude,
				longitude: dms_longitude
			}
			encoded[:irradiation] = dms_hash[:azimuth] if dms_hash[:azimuth]
		end

		return encoded 
	end
end