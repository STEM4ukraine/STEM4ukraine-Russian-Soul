ha:cschem-sheet-v1 {
	ha:obj_indirect.1 {
		li:objects {
			ha:group.1 {
				uuid=LjOMK/PbzYK/2HBB46gAAAAN;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAAAO; loclib_name=led5;
						li:objects {
						}
						ha:attrib {
							device=led5
							footprint=LED5
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAAC6; loclib_name=3mmLED_backplane;
						li:objects {
						}
						ha:attrib {
							footprint=3mmLEDbackplane.lht
							li:portmap {
								{C->pcb/pinnum=1}
								{A->pcb/pinnum=2}
							}
						}
					}
				}
				ha:attrib {
					ha:purpose = { value=devmap; prio=0; }
				}
			}
		}
	}
	ha:obj_direct.2 {
		uuid=LjOMK/PbzYK/2HBB46gAAAAC;
		li:objects {
			ha:pen.sheet-decor { shape=round; size=125; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.sheet-decor-fill { shape=round; size=125; color=#bbbbbb; font_height=3000; font_family=sans; }
			ha:pen.titlebox-frame { shape=round; size=250; color=#777777; font_height=0; }
			ha:pen.titlebox-fill { shape=round; size=250; color=#bbffbb; font_height=0; }
			ha:pen.titlebox-big { shape=round; size=250; color=#777777; font_height=3000; font_family=sans; }
			ha:pen.titlebox-small { shape=round; size=250; color=#777777; font_height=1500; font_family=sans; }
			ha:pen.wire { shape=round; size=250; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.bus { shape=round; size=1500; color=#2222bb; font_height=3000; font_family=sans; }
			ha:pen.hub { shape=round; size=3000; color=#6666ff; font_height=3000; font_family=sans; }
			ha:pen.sym-decor { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; }
			ha:pen.sym-decor-fill { shape=round; size=125; color=#99ff99; font_height=3000; font_family=sans; }
			ha:pen.sym-primary { shape=round; size=125; color=#119911; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.sym-secondary { shape=round; size=125; color=#33bb33; font_height=3000; font_family=sans; }
			ha:pen.term-decor { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.term-primary { shape=round; size=250; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.term-secondary { shape=round; size=250; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.busterm-decor { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; }
			ha:pen.busterm-primary { shape=round; size=1500; color=#222222; font_height=3000; font_family=sans; font_style=bold; }
			ha:pen.busterm-secondary { shape=round; size=1500; color=#555555; font_height=3000; font_family=sans; }
			ha:pen.junction { shape=round; size=1000; color=#2222bb; font_height=3000; font_family=sans; }
			ha:group.4 {
				uuid=LjOMK/PbzYK/2HBB46gAAAAY; src_uuid=iNOQfJpO6hT/HFDFGjoAAABX;
				x=16000; y=80000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAAAZ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABY;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=N
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAAAa; src_uuid=iNOQfJpO6hT/HFDFGjoAAABZ;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=P
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=9000; y1=5000; x2=9000; y2=-5000; stroke=sym-decor; }
					ha:line.4 { x1=11000; y1=2000; x2=11000; y2=-2000; stroke=sym-decor; }
					ha:line.5 { x1=4000; y1=0; x2=9000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=11000; y1=0; x2=16000; y2=0; stroke=sym-decor; }
					ha:text.7 { x1=5000; y1=5000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					footprint=CR2032-battery-holder-v1.rf
					name=B1
					role=symbol
				}
			}
			ha:group.56 {
				uuid=LjOMK/PbzYK/2HBB46gAAACp;
				x=32000; y=88000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAACq; src_uuid=LjOMK/PbzYK/2HBB46gAAABe;
						x=4000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:line.2 { x1=4000; y1=0; x2=7200; y2=0; stroke=sym-decor; }
					ha:group.3 {
						uuid=LjOMK/PbzYK/2HBB46gAAACr; src_uuid=LjOMK/PbzYK/2HBB46gAAABf;
						x=16000; y=-4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=3
							role=terminal
						}
					}
					ha:line.4 { x1=12800; y1=-4000; x2=16000; y2=-4000; stroke=sym-decor; }
					ha:arc.5 { cx=12400; cy=-4000; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:group.6 {
						uuid=LjOMK/PbzYK/2HBB46gAAACs; src_uuid=LjOMK/PbzYK/2HBB46gAAABg;
						x=16000; y=4000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:line.7 { x1=12800; y1=4000; x2=16000; y2=4000; stroke=sym-decor; }
					ha:arc.8 { cx=12400; cy=4000; r=400; sang=0.000000; dang=360.000000; stroke=sym-decor; }
					ha:line.9 { x1=7200; y1=0; x2=11600; y2=3200; stroke=sym-decor; }
					ha:group.10 {
						uuid=LjOMK/PbzYK/2HBB46gAAACt; src_uuid=LjOMK/PbzYK/2HBB46gAAABe;
						x=4000; y=-8000; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=1000; y1=0; dyntext=1; stroke=term-primary; text=%../A.name%; }
						}
						ha:attrib {
							name=4
							role=terminal
						}
					}
					ha:line.11 { x1=6000; y1=3500; x2=6000; y2=2500; stroke=sym-decor; }
					ha:line.12 { x1=6000; y1=1500; x2=6000; y2=500; stroke=sym-decor; }
					ha:line.13 { x1=6000; y1=-500; x2=6000; y2=-1500; stroke=sym-decor; }
					ha:line.14 { x1=6000; y1=-2500; x2=6000; y2=-3500; stroke=sym-decor; }
					ha:line.15 { x1=6000; y1=-4500; x2=6000; y2=-5500; stroke=sym-decor; }
					ha:line.16 { x1=6000; y1=-6500; x2=6000; y2=-7500; stroke=sym-decor; }
					ha:line.17 { x1=14000; y1=3500; x2=14000; y2=2500; stroke=sym-decor; }
					ha:line.18 { x1=14000; y1=1500; x2=14000; y2=500; stroke=sym-decor; }
					ha:line.19 { x1=14000; y1=-500; x2=14000; y2=-1500; stroke=sym-decor; }
					ha:line.20 { x1=14000; y1=-2500; x2=14000; y2=-3500; stroke=sym-decor; }
					ha:line.21 { x1=14000; y1=-4500; x2=14000; y2=-5500; stroke=sym-decor; }
					ha:line.22 { x1=14000; y1=-6500; x2=14000; y2=-7500; stroke=sym-decor; }
					ha:line.23 { x1=14000; y1=5500; x2=14000; y2=4500; stroke=sym-decor; }
					ha:line.24 { x1=6000; y1=5500; x2=6000; y2=4500; stroke=sym-decor; }
					ha:line.25 { x1=7500; y1=6000; x2=6500; y2=6000; stroke=sym-decor; }
					ha:line.26 { x1=9500; y1=6000; x2=8500; y2=6000; stroke=sym-decor; }
					ha:line.27 { x1=11500; y1=6000; x2=10500; y2=6000; stroke=sym-decor; }
					ha:line.28 { x1=13500; y1=6000; x2=12500; y2=6000; stroke=sym-decor; }
					ha:line.29 { x1=7500; y1=-10000; x2=6500; y2=-10000; stroke=sym-decor; }
					ha:line.30 { x1=9500; y1=-10000; x2=8500; y2=-10000; stroke=sym-decor; }
					ha:line.31 { x1=11500; y1=-10000; x2=10500; y2=-10000; stroke=sym-decor; }
					ha:line.32 { x1=13500; y1=-10000; x2=12500; y2=-10000; stroke=sym-decor; }
					ha:line.33 { x1=6000; y1=-8500; x2=6000; y2=-9500; stroke=sym-decor; }
					ha:line.34 { x1=14000; y1=-8500; x2=14000; y2=-9500; stroke=sym-decor; }
					ha:line.35 { x1=4000; y1=-8000; x2=6000; y2=-8000; stroke=sym-decor; }
					ha:text.36 { x1=6000; y1=6000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
				}
				ha:attrib {
					footprint=MSK12C02-SMD-switch-v1.rf
					name=SW1
					role=symbol
				}
			}
			ha:group.57 {
				uuid=LjOMK/PbzYK/2HBB46gAAADH; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=88000; y=48000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADI; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADJ; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane
					name=LED2
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.58 {
				uuid=LjOMK/PbzYK/2HBB46gAAADK; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAQ;
				x=64000; y=48000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADL; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAR;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=C
							role=terminal
							ha:spice/pinnum = { value=2; prio=31050; }
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADM; src_uuid=iNOQfJpO6hT/HFDFGjoAAAAS;
						x=-16000; y=0; mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=A
							role=terminal
							ha:spice/pinnum = { value=1; prio=31050; }
						}
					}
					ha:line.3 { x1=-4000; y1=0; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.4 { x1=-12000; y1=0; x2=-10000; y2=0; stroke=sym-decor; }
					ha:line.5 { x1=-10000; y1=4000; x2=-6000; y2=0; stroke=sym-decor; }
					ha:line.6 { x1=-6000; y1=0; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.7 { x1=-10000; y1=4000; x2=-10000; y2=-4000; stroke=sym-decor; }
					ha:line.8 { x1=-6000; y1=4000; x2=-6000; y2=-4000; stroke=sym-decor; }
					ha:text.10 { x1=-8000; y1=13000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:line.11 { x1=-8000; y1=8000; x2=-6000; y2=11000; stroke=sym-decor; }
					ha:line.12 { x1=-6000; y1=11000; x2=-7000; y2=10000; stroke=sym-decor; }
					ha:line.13 { x1=-6000; y1=11000; x2=-6517; y2=9545; stroke=sym-decor; }
					ha:line.14 { x1=-10000; y1=7000; x2=-8000; y2=10000; stroke=sym-decor; }
					ha:line.15 { x1=-8000; y1=10000; x2=-8000; y2=8000; stroke=sym-decor; }
					ha:line.16 { x1=-8303; y1=6354; x2=-6303; y2=9354; stroke=sym-decor; }
					ha:line.17 { x1=-6303; y1=9354; x2=-7303; y2=8354; stroke=sym-decor; }
					ha:line.18 { x1=-6303; y1=9354; x2=-6820; y2=7899; stroke=sym-decor; }
					ha:line.19 { x1=-10303; y1=5354; x2=-8303; y2=8354; stroke=sym-decor; }
					ha:line.20 { x1=-8303; y1=8354; x2=-8303; y2=6354; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					devmap=3mmLED_backplane
					name=LED1
					role=symbol
					ha:spice/prefix = { value=D; prio=31050; }
				}
			}
			ha:group.59 {
				uuid=LjOMK/PbzYK/2HBB46gAAADN; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=64000; y=88000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADO; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADP; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R1
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=120
				}
			}
			ha:group.60 {
				uuid=LjOMK/PbzYK/2HBB46gAAADQ; src_uuid=iNOQfJpO6hT/HFDFGjoAAABC;
				x=88000; y=88000; rot=270.000000;
				li:objects {
					ha:group.1 {
						uuid=LjOMK/PbzYK/2HBB46gAAADR; src_uuid=iNOQfJpO6hT/HFDFGjoAAABD;
						x=20000; y=0;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=2
							role=terminal
						}
					}
					ha:group.2 {
						uuid=LjOMK/PbzYK/2HBB46gAAADS; src_uuid=iNOQfJpO6hT/HFDFGjoAAABE;
						mirx=1;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
							ha:text.2 { x1=-3000; y1=0; dyntext=1; stroke=term-primary; text=%../a.display/name%; }
						}
						ha:attrib {
							name=1
							role=terminal
						}
					}
					ha:text.3 { x1=12000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../a.value%; floater=1; }
					ha:text.4 { x1=8000; y1=3000; rot=90.000000; dyntext=1; stroke=sym-primary; text=%../A.name%; floater=1; }
					ha:polygon.5 {
						li:outline {
							ha:line { x1=4000; y1=2000; x2=4000; y2=-2000; }
							ha:line { x1=4000; y1=-2000; x2=16000; y2=-2000; }
							ha:line { x1=16000; y1=-2000; x2=16000; y2=2000; }
							ha:line { x1=16000; y1=2000; x2=4000; y2=2000; }
						}
						stroke=sym-decor;
					}
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					ha:device = { value=resistor; prio=31050; }
					footprint=acy(300)
					name=R2
					role=symbol
					ha:spice/prefix = { value=R; prio=31050; }
					value=120
				}
			}
			ha:group.61 {
				uuid=LjOMK/PbzYK/2HBB46gAAADT;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=52000; y1=84000; x2=88000; y2=84000; stroke=wire; }
					ha:line.2 { x1=88000; y1=84000; x2=88000; y2=80000; stroke=wire; }
					ha:line.3 { x1=64000; y1=84000; x2=64000; y2=80000; stroke=wire; }
					ha:line.4 { x1=64000; y1=84000; x2=64000; y2=84000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.63 {
				li:conn {
					/2/61/2
					/2/60/2/1
				}
			}
			ha:connection.64 {
				li:conn {
					/2/61/3
					/2/59/2/1
				}
			}
			ha:group.65 {
				uuid=LjOMK/PbzYK/2HBB46gAAADU;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=64000; y1=56000; x2=64000; y2=60000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.66 {
				li:conn {
					/2/65/1
					/2/58/2/1
				}
			}
			ha:group.68 {
				uuid=LjOMK/PbzYK/2HBB46gAAADV;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=88000; y1=56000; x2=88000; y2=60000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.70 {
				li:conn {
					/2/68/1
					/2/57/2/1
				}
			}
			ha:group.75 {
				uuid=LjOMK/PbzYK/2HBB46gAAADX;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=16000; y1=72000; x2=16000; y2=80000; stroke=wire; }
					ha:line.2 { x1=16000; y1=80000; x2=32000; y2=80000; stroke=wire; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.76 {
				li:conn {
					/2/75/1
					/2/4/2/1
				}
			}
			ha:group.78 {
				uuid=LjOMK/PbzYK/2HBB46gAAADY;
				x=0; y=8000;
				li:objects {
					ha:line.1 { x1=32000; y1=72000; x2=28000; y2=72000; stroke=wire; }
					ha:line.2 { x1=16000; y1=52000; x2=16000; y2=36000; stroke=wire; }
					ha:line.3 { x1=64000; y1=36000; x2=64000; y2=40000; stroke=wire; }
					ha:line.4 { x1=16000; y1=36000; x2=88000; y2=36000; stroke=wire; }
					ha:line.5 { x1=64000; y1=36000; x2=64000; y2=36000; stroke=junction; }
					ha:line.6 { x1=88000; y1=36000; x2=88000; y2=40000; stroke=wire; }
					ha:line.7 { x1=28000; y1=72000; x2=28000; y2=36000; stroke=wire; }
					ha:line.8 { x1=28000; y1=36000; x2=28000; y2=36000; stroke=junction; }
				}
				ha:attrib {
					ha:role = { value=wire-net; prio=0; }
				}
			}
			ha:connection.84 {
				li:conn {
					/2/61/1
					/2/56/6/1
				}
			}
			ha:connection.85 {
				li:conn {
					/2/65/1
					/2/59/1/1
				}
			}
			ha:connection.86 {
				li:conn {
					/2/68/1
					/2/60/1/1
				}
			}
			ha:connection.87 {
				li:conn {
					/2/75/2
					/2/56/1/1
				}
			}
			ha:connection.88 {
				li:conn {
					/2/78/1
					/2/56/10/1
				}
			}
			ha:connection.89 {
				li:conn {
					/2/78/2
					/2/4/1/1
				}
			}
			ha:connection.90 {
				li:conn {
					/2/78/3
					/2/58/1/1
				}
			}
			ha:connection.91 {
				li:conn {
					/2/78/6
					/2/57/1/1
				}
			}
			ha:group.92 {
				uuid=ukggpUEc4cgsieO3pZUAAAAh; src_uuid=iNOQfJpO6hT/HFDFGjoAAABm;
				x=16000; y=44000;
				li:objects {
					ha:group.1 {
						uuid=ukggpUEc4cgsieO3pZUAAAAi; src_uuid=iNOQfJpO6hT/HFDFGjoAAABn;
						rot=90.000000;
						li:objects {
							ha:line.1 { x1=0; y1=0; x2=-4000; y2=0; stroke=term-decor; }
						}
						ha:attrib {
							drc/require_graphical_conn=1
							ha:name = { value=1; prio=220; }
							role=terminal
						}
					}
					ha:line.2 { x1=-1500; y1=-5000; x2=1500; y2=-5000; stroke=sym-decor; }
					ha:line.3 { x1=-500; y1=-6000; x2=500; y2=-6000; stroke=sym-decor; }
					ha:line.4 { x1=-2500; y1=-4000; x2=2500; y2=-4000; stroke=sym-decor; }
				}
				ha:attrib {
					-sym-copyright=(C) 2022 Tibor 'Igor2' Palinkas
					-sym-license-dist=GPLv2+
					-sym-license-use=Public Domain
					-sym-source=sch-rnd default symbol lib
					li:connect {
						{1:GND}
					}
					role=symbol
				}
			}
			ha:connection.93 {
				li:conn {
					/2/92/1/1
					/2/78/2
					/2/78/4
				}
			}
		}
		ha:attrib {
			drawing_min_height=200000
			drawing_min_width=287000
			maintainer=<maint. attr>
			page=<page attr>
			print_page=A/4
			title=<please set sheet title attribute>
		}
	}
  li:sch-rnd-conf-v1 {
   ha:overwrite {
    ha:editor {
     grids_idx = 2
     grid = 4.0960 mm
    }
   }
  }
}
