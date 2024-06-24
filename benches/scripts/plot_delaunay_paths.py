from plot_delaunay import plot_delaunay_stats

OUTER_CONSTANT = 15.5
INNER_CONSTANT = 30_000_000

sizes = [1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000, 11000, 12000, 13000, 14000, 15000, 16000, 17000, 18000, 19000, 20000, 21000, 22000, 23000, 24000, 25000, 26000, 27000, 28000, 29000, 30000, 31000, 32000, 33000, 34000, 35000, 36000, 37000, 38000, 39000, 40000, 41000, 42000, 43000, 44000, 45000, 46000, 47000, 48000, 49000, 50000, 51000, 52000, 53000, 54000, 55000, 56000, 57000, 58000, 59000, 60000, 61000, 62000, 63000, 64000, 65000, 66000, 67000, 68000, 69000, 70000, 71000, 72000, 73000, 74000, 75000, 76000, 77000, 78000, 79000, 80000, 81000, 82000, 83000, 84000, 85000, 86000, 87000, 88000, 89000, 90000, 91000, 92000, 93000, 94000, 95000, 96000, 97000, 98000, 99000, 100000, 101000, 102000, 103000, 104000, 105000, 106000, 107000, 108000, 109000, 110000, 111000, 112000, 113000, 114000, 115000, 116000, 117000, 118000, 119000, 120000, 121000, 122000, 123000, 124000, 125000, 126000, 127000, 128000, 129000, 130000, 131000, 132000, 133000, 134000, 135000, 136000, 137000, 138000, 139000, 140000, 141000, 142000, 143000, 144000, 145000, 146000, 147000, 148000, 149000, 150000, 151000, 152000, 153000, 154000, 155000, 156000, 157000, 158000, 159000, 160000, 161000, 162000, 163000, 164000, 165000, 166000, 167000, 168000, 169000, 170000, 171000, 172000, 173000, 174000, 175000, 176000, 177000, 178000, 179000, 180000, 181000, 182000, 183000, 184000, 185000, 186000, 187000, 188000, 189000, 190000, 191000, 192000, 193000, 194000, 195000, 196000, 197000, 198000, 199000, 200000]
times = [914129.0276995798, 1782027.3348682239, 2766564.044463119, 4488968.031895314, 4850880.584578112, 6614060.779560847, 7784264.47081746, 9088395.514011543, 11566150.059058957, 10929996.764506172, 14621805.137885487, 13900591.868265307, 17133895.805674605, 18539684.100420635, 19935109.2565, 20391629.480039682, 25003123.67844246, 24694273.826170634, 26345511.48469246, 30773742.273472227, 32694066.032248676, 31331356.55272486, 34348284.902209, 37023707.52498677, 40039444.43376984, 36455484.058425926, 38321098.01294973, 44192981.010238096, 42489814.903531745, 46484861.741044976, 45727792.19732143, 46700727.655079365, 49073990.92904762, 53810665.597480156, 52456729.17355158, 56230686.327063486, 59854467.10861112, 58871072.05579364, 65204044.22960317, 62873030.90886905, 65377721.77791667, 70612876.32904762, 75215549.22611111, 76149339.81418651, 78227981.19797619, 76075962.03882937, 76909874.42625001, 80595375.94424602, 82282553.80716269, 82824986.70738095, 84588277.37277779, 86355112.4075, 89638470.64412698, 90913696.25940475, 98463107.76039684, 93998606.01904762, 96054852.33210316, 96709456.5429365, 106904015.5159524, 101892488.66313493, 109850393.46305557, 108833084.52369049, 113609839.49670634, 109028785.97142859, 115396726.52301589, 112091904.90309525, 121535248.4476984, 125578578.89222224, 125495290.84785715, 120474599.30337302, 131769769.62376983, 124223913.88293651, 127496146.02261905, 141424667.7220238, 130623470.691627, 136270664.5174603, 144374475.17468253, 136511860.3747619, 144402165.93285716, 145778421.26833335, 143493313.64349204, 150511359.70027775, 147752524.83595237, 161676161.21940476, 152621938.92123017, 151857076.47710317, 167697282.27873015, 163433426.49063495, 173762201.15142858, 173124239.32690474, 177725406.49031743, 168052400.86821428, 168720087.65460315, 177655639.26301587, 178917317.9877381, 192087739.53333336, 185962644.03333333, 187326540.43333334, 183866028.93333334, 179497046.89746034, 183315548.16666666, 186223593.36666664, 200332216.86666664, 204623964.56666666, 202137956.13333336, 193440613.53333336, 207214506.93333334, 197779030.3333333, 211814883.46666667, 204530538.9333333, 201039212.33333334, 220692874.00000006, 214631881.23333332, 223118551.16666666, 216722593.4, 212283628.7, 216022807.0666667, 222444613.43333334, 221481736.2666667, 238456041.7, 242228975.7, 242048132.4, 231587466.8666667, 254323696.15, 252953439.45, 255189065.35, 246577485.8, 257913035.45, 238431124.0, 255964924.6, 239598191.6, 260726952.45, 259659488.65, 249695163.2666667, 257103588.9, 268905438.55, 253485065.0, 273419050.7, 273649479.9, 265725279.7, 261937086.85, 286704224.75, 287130473.25, 278615157.0, 274481812.1, 296217725.8, 281031957.2, 303396312.4, 303498264.55, 295598547.1, 286404508.35, 307972399.75, 302516496.1, 307145586.45, 308917811.3, 291482585.7, 318562540.25, 316637341.5, 297314515.05, 319777681.85, 319331385.25, 325132192.45, 332098174.8, 333940314.5, 324083988.7, 318719636.15, 321504431.65, 329833140.0, 330128788.6, 333381347.1, 339068657.1, 328971328.1, 346540263.8, 344881212.4, 340531964.95, 335894743.8, 359052985.95, 364759428.55, 369973275.4, 370897925.65, 347243554.2, 377170208.85, 357359986.8, 383949350.2, 386487107.6, 387482538.15, 371352016.3, 381807202.85, 362431491.4, 388460314.0, 362135301.45, 375778163.5, 393197285.8, 382018711.3, 407331227.75, 400918971.6, 392267096.1, 390363191.9, 388329190.7, 409451457.8]

plot_delaunay_stats(
    "shortest odd path",
    sizes,
    times,
    OUTER_CONSTANT,
    INNER_CONSTANT
)