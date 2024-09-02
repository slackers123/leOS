let path;

let rx_edit;
let ry_edit;
let x_axis_rotation_edit;
let large_arc_flag_edit;
let sweep_flag_edit;
let x_edit;
let y_edit;

let rx = 30;
let ry = 50;
let x_axis_rotation = 0;
let large_arc_flag = false;
let sweep_flag = true;
let x = 162.55;
let y = 162.45;

function set_attrib() {
  let new_attrib = `M 10 315
 L 110 215
 a${rx} ${ry} ${x_axis_rotation} ${large_arc_flag ? 1 : 0} ${sweep_flag ? 1 : 0} ${x} ${y}
 L 315 10`;
  console.log(new_attrib);
  path.setAttribute("d", new_attrib);
}

function set_range(target, val) {
  target.value = val;
}

function range_setup(target, min, max, val) {
  target.setAttribute("min", min);
  target.setAttribute("max", max);
  set_range(target, val);
}

function set_flag(target, val) {
  target.checked = val;
}

window.onload = () => {
  path = document.getElementById("path");

  rx_edit = document.getElementById("rx");
  ry_edit = document.getElementById("ry");
  x_axis_rotation_edit = document.getElementById("x-axis-rotation");
  large_arc_flag_edit = document.getElementById("large-arc-flag");
  sweep_flag_edit = document.getElementById("sweep-flag");
  x_edit = document.getElementById("x");
  y_edit = document.getElementById("y");

  console.log("rx_edit", rx_edit);
  console.log("ry_edit", ry_edit);
  console.log("x_axis_rotation_edit", x_axis_rotation_edit);
  console.log("large_arc_flag_edit", large_arc_flag_edit);
  console.log("sweep_flag_edit", sweep_flag_edit);
  console.log("x_edit", x_edit);
  console.log("y_edit", y_edit);

  range_setup(rx_edit, 0, 360, rx);
  range_setup(ry_edit, 0, 360, ry);
  range_setup(x_axis_rotation_edit, 0, 360, x_axis_rotation);
  set_flag(large_arc_flag_edit, large_arc_flag);
  set_flag(sweep_flag_edit, sweep_flag);
  range_setup(x_edit, 0, 360, x);
  range_setup(y_edit, 0, 360, y);

  rx_edit.oninput = () => {
    rx = rx_edit.value;
    set_attrib();
  };
  ry_edit.oninput = () => {
    ry = ry_edit.value;
    set_attrib();
  };
  x_axis_rotation_edit.oninput = () => {
    x_axis_rotation = x_axis_rotation_edit.value;
    console.log(x_axis_rotation);
    set_attrib();
  };
  large_arc_flag_edit.oninput = () => {
    large_arc_flag = large_arc_flag_edit.checked;
    set_attrib();
  };
  sweep_flag_edit.oninput = () => {
    sweep_flag = sweep_flag_edit.checked;
    set_attrib();
  };
  x_edit.oninput = () => {
    x = x_edit.value;
    set_attrib();
  };
  y_edit.oninput = () => {
    y = y_edit.value;
    set_attrib();
  };

  set_attrib();
};
