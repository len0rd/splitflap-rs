$fn=25;

bracket_height = 4;

length = 200;
width = 31;
actual_length = 198.12;
actual_width = 30.48;
length_buffer = length - actual_length;
width_buffer = width - actual_width;
length_buffer_per_side = length_buffer / 2;
width_buffer_per_side = width_buffer / 2;

mount_hole_radius = 1.5;
mount_hole_center_to_edge = 3.3;

motor_cutout_start_y = 1.75;
motor_cutout_end_y = 10.5;
motor_cutout_start_x = 18;

sensor_cutout_start_y = 19.25;
sensor_cutout_end_y =  23.75;
sensor_cutout_start_x = 23.25;
sensor_mid_gap_start = 71;

ribbon_cutout_start_y = 7.35;
ribbon_cutout_end_y = 22.5;
ribbon_cutout_start_x = 7.5;
ribbon_cutout_len = 7;


difference(){
    roundedcube([length, width, bracket_height], false, 2, "z");
    
    // punch out mounting holes
    translate([
        length_buffer_per_side + mount_hole_center_to_edge, 
        width_buffer_per_side + mount_hole_center_to_edge, 
        0])
    cylinder(h=bracket_height, r=mount_hole_radius);
    
    translate([
        length - (length_buffer_per_side + mount_hole_center_to_edge), 
        width_buffer_per_side + mount_hole_center_to_edge, 
        0])
    cylinder(h=bracket_height, r=mount_hole_radius);
    
    translate([
        length - (length_buffer_per_side + mount_hole_center_to_edge), 
        width - (width_buffer_per_side + mount_hole_center_to_edge), 
        0])
    cylinder(h=bracket_height, r=mount_hole_radius);
    
    translate([
        length_buffer_per_side + mount_hole_center_to_edge,
        width - (width_buffer_per_side + mount_hole_center_to_edge),
        0])
    cylinder(h=bracket_height, r=mount_hole_radius);
    
    // puch out through-hole solder areas
    
    // motor and power
    motor_cutout_len = actual_length - (motor_cutout_start_x*2);
    motor_cutout_width = motor_cutout_end_y - motor_cutout_start_y;
    translate([
        length_buffer_per_side + motor_cutout_start_x,
        width_buffer_per_side + motor_cutout_start_y,
        0])
    roundedcube([motor_cutout_len, motor_cutout_width, bracket_height], false, 1, "z");
    
    // sensor A
    sensor_cutout_len = sensor_mid_gap_start - sensor_cutout_start_x;
    sensor_cutout_width = sensor_cutout_end_y - sensor_cutout_start_y;
    translate([
        length_buffer_per_side + sensor_cutout_start_x,
        width_buffer_per_side + sensor_cutout_start_y,
        0])
    roundedcube([sensor_cutout_len, sensor_cutout_width, bracket_height], false, 1, "z");
    
    // sensor B
    translate([
        length - (length_buffer_per_side + sensor_cutout_start_x + sensor_cutout_len),
        width_buffer_per_side + sensor_cutout_start_y,
        0])
    roundedcube([sensor_cutout_len, sensor_cutout_width, bracket_height], false, 1, "z");
    
    // ribbon A
    ribbon_cutout_width = ribbon_cutout_end_y - ribbon_cutout_start_y;
    translate([
        length_buffer_per_side + ribbon_cutout_start_x,
        width_buffer_per_side + ribbon_cutout_start_y,
        0])
    roundedcube([ribbon_cutout_len, ribbon_cutout_width, bracket_height], false, 1, "z");
    
    // ribbon B
    translate([
        length - (length_buffer_per_side + ribbon_cutout_start_x + ribbon_cutout_len),
        width_buffer_per_side + ribbon_cutout_start_y,
        0])
    roundedcube([ribbon_cutout_len, ribbon_cutout_width, bracket_height], false, 1, "z");
}

// Higher definition curves
// FROM: https://danielupshaw.com/openscad-rounded-corners/
$fs = 0.01;

module roundedcube(size = [1, 1, 1], center = false, radius = 0.5, apply_to = "all") {
	// If single value, convert to [x, y, z] vector
	size = (size[0] == undef) ? [size, size, size] : size;

	translate_min = radius;
	translate_xmax = size[0] - radius;
	translate_ymax = size[1] - radius;
	translate_zmax = size[2] - radius;

	diameter = radius * 2;

	module build_point(type = "sphere", rotate = [0, 0, 0]) {
		if (type == "sphere") {
			sphere(r = radius);
		} else if (type == "cylinder") {
			rotate(a = rotate)
			cylinder(h = diameter, r = radius, center = true);
		}
	}

	obj_translate = (center == false) ?
		[0, 0, 0] : [
			-(size[0] / 2),
			-(size[1] / 2),
			-(size[2] / 2)
		];

	translate(v = obj_translate) {
		hull() {
			for (translate_x = [translate_min, translate_xmax]) {
				x_at = (translate_x == translate_min) ? "min" : "max";
				for (translate_y = [translate_min, translate_ymax]) {
					y_at = (translate_y == translate_min) ? "min" : "max";
					for (translate_z = [translate_min, translate_zmax]) {
						z_at = (translate_z == translate_min) ? "min" : "max";

						translate(v = [translate_x, translate_y, translate_z])
						if (
							(apply_to == "all") ||
							(apply_to == "xmin" && x_at == "min") || (apply_to == "xmax" && x_at == "max") ||
							(apply_to == "ymin" && y_at == "min") || (apply_to == "ymax" && y_at == "max") ||
							(apply_to == "zmin" && z_at == "min") || (apply_to == "zmax" && z_at == "max")
						) {
							build_point("sphere");
						} else {
							rotate = 
								(apply_to == "xmin" || apply_to == "xmax" || apply_to == "x") ? [0, 90, 0] : (
								(apply_to == "ymin" || apply_to == "ymax" || apply_to == "y") ? [90, 90, 0] :
								[0, 0, 0]
							);
							build_point("cylinder", rotate);
						}
					}
				}
			}
		}
	}
}
