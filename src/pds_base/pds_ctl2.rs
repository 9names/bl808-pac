#[doc = "Register `pds_ctl2` reader"]
pub struct R(crate::R<PDS_CTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDS_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDS_CTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDS_CTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `pds_ctl2` writer"]
pub struct W(crate::W<PDS_CTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDS_CTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PDS_CTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDS_CTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `reserved_0` reader - "]
pub type RESERVED_0_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_mm_pwr_off` reader - "]
pub type CR_PDS_FORCE_MM_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_mm_pwr_off` writer - "]
pub type CR_PDS_FORCE_MM_PWR_OFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `reserved_2` reader - "]
pub type RESERVED_2_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_usb_pwr_off` reader - "]
pub type CR_PDS_FORCE_USB_PWR_OFF_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_usb_pwr_off` writer - "]
pub type CR_PDS_FORCE_USB_PWR_OFF_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `reserved_4` reader - "]
pub type RESERVED_4_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_mm_iso_en` reader - "]
pub type CR_PDS_FORCE_MM_ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_mm_iso_en` writer - "]
pub type CR_PDS_FORCE_MM_ISO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `reserved_6` reader - "]
pub type RESERVED_6_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_usb_iso_en` reader - "]
pub type CR_PDS_FORCE_USB_ISO_EN_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_usb_iso_en` writer - "]
pub type CR_PDS_FORCE_USB_ISO_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_np_pds_rst` reader - "]
pub type CR_PDS_FORCE_NP_PDS_RST_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_np_pds_rst` writer - "]
pub type CR_PDS_FORCE_NP_PDS_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_mm_pds_rst` reader - "]
pub type CR_PDS_FORCE_MM_PDS_RST_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_mm_pds_rst` writer - "]
pub type CR_PDS_FORCE_MM_PDS_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_wb_pds_rst` reader - "]
pub type CR_PDS_FORCE_WB_PDS_RST_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_wb_pds_rst` writer - "]
pub type CR_PDS_FORCE_WB_PDS_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_usb_pds_rst` reader - "]
pub type CR_PDS_FORCE_USB_PDS_RST_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_usb_pds_rst` writer - "]
pub type CR_PDS_FORCE_USB_PDS_RST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_np_mem_stby` reader - "]
pub type CR_PDS_FORCE_NP_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_np_mem_stby` writer - "]
pub type CR_PDS_FORCE_NP_MEM_STBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_mm_mem_stby` reader - "]
pub type CR_PDS_FORCE_MM_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_mm_mem_stby` writer - "]
pub type CR_PDS_FORCE_MM_MEM_STBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_wb_mem_stby` reader - "]
pub type CR_PDS_FORCE_WB_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_wb_mem_stby` writer - "]
pub type CR_PDS_FORCE_WB_MEM_STBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_usb_mem_stby` reader - "]
pub type CR_PDS_FORCE_USB_MEM_STBY_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_usb_mem_stby` writer - "]
pub type CR_PDS_FORCE_USB_MEM_STBY_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_np_gate_clk` reader - "]
pub type CR_PDS_FORCE_NP_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_np_gate_clk` writer - "]
pub type CR_PDS_FORCE_NP_GATE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_mm_gate_clk` reader - "]
pub type CR_PDS_FORCE_MM_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_mm_gate_clk` writer - "]
pub type CR_PDS_FORCE_MM_GATE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_wb_gate_clk` reader - "]
pub type CR_PDS_FORCE_WB_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_wb_gate_clk` writer - "]
pub type CR_PDS_FORCE_WB_GATE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `cr_pds_force_usb_gate_clk` reader - "]
pub type CR_PDS_FORCE_USB_GATE_CLK_R = crate::BitReader<bool>;
#[doc = "Field `cr_pds_force_usb_gate_clk` writer - "]
pub type CR_PDS_FORCE_USB_GATE_CLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PDS_CTL2_SPEC, bool, O>;
#[doc = "Field `reserved_20_31` reader - "]
pub type RESERVED_20_31_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reserved_0(&self) -> RESERVED_0_R {
        RESERVED_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cr_pds_force_mm_pwr_off(&self) -> CR_PDS_FORCE_MM_PWR_OFF_R {
        CR_PDS_FORCE_MM_PWR_OFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn reserved_2(&self) -> RESERVED_2_R {
        RESERVED_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pwr_off(&self) -> CR_PDS_FORCE_USB_PWR_OFF_R {
        CR_PDS_FORCE_USB_PWR_OFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn reserved_4(&self) -> RESERVED_4_R {
        RESERVED_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn cr_pds_force_mm_iso_en(&self) -> CR_PDS_FORCE_MM_ISO_EN_R {
        CR_PDS_FORCE_MM_ISO_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn reserved_6(&self) -> RESERVED_6_R {
        RESERVED_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn cr_pds_force_usb_iso_en(&self) -> CR_PDS_FORCE_USB_ISO_EN_R {
        CR_PDS_FORCE_USB_ISO_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn cr_pds_force_np_pds_rst(&self) -> CR_PDS_FORCE_NP_PDS_RST_R {
        CR_PDS_FORCE_NP_PDS_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn cr_pds_force_mm_pds_rst(&self) -> CR_PDS_FORCE_MM_PDS_RST_R {
        CR_PDS_FORCE_MM_PDS_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cr_pds_force_wb_pds_rst(&self) -> CR_PDS_FORCE_WB_PDS_RST_R {
        CR_PDS_FORCE_WB_PDS_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn cr_pds_force_usb_pds_rst(&self) -> CR_PDS_FORCE_USB_PDS_RST_R {
        CR_PDS_FORCE_USB_PDS_RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn cr_pds_force_np_mem_stby(&self) -> CR_PDS_FORCE_NP_MEM_STBY_R {
        CR_PDS_FORCE_NP_MEM_STBY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn cr_pds_force_mm_mem_stby(&self) -> CR_PDS_FORCE_MM_MEM_STBY_R {
        CR_PDS_FORCE_MM_MEM_STBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn cr_pds_force_wb_mem_stby(&self) -> CR_PDS_FORCE_WB_MEM_STBY_R {
        CR_PDS_FORCE_WB_MEM_STBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn cr_pds_force_usb_mem_stby(&self) -> CR_PDS_FORCE_USB_MEM_STBY_R {
        CR_PDS_FORCE_USB_MEM_STBY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn cr_pds_force_np_gate_clk(&self) -> CR_PDS_FORCE_NP_GATE_CLK_R {
        CR_PDS_FORCE_NP_GATE_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn cr_pds_force_mm_gate_clk(&self) -> CR_PDS_FORCE_MM_GATE_CLK_R {
        CR_PDS_FORCE_MM_GATE_CLK_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn cr_pds_force_wb_gate_clk(&self) -> CR_PDS_FORCE_WB_GATE_CLK_R {
        CR_PDS_FORCE_WB_GATE_CLK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn cr_pds_force_usb_gate_clk(&self) -> CR_PDS_FORCE_USB_GATE_CLK_R {
        CR_PDS_FORCE_USB_GATE_CLK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31"]
    #[inline(always)]
    pub fn reserved_20_31(&self) -> RESERVED_20_31_R {
        RESERVED_20_31_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_mm_pwr_off(&mut self) -> CR_PDS_FORCE_MM_PWR_OFF_W<1> {
        CR_PDS_FORCE_MM_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_pwr_off(&mut self) -> CR_PDS_FORCE_USB_PWR_OFF_W<3> {
        CR_PDS_FORCE_USB_PWR_OFF_W::new(self)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_mm_iso_en(&mut self) -> CR_PDS_FORCE_MM_ISO_EN_W<5> {
        CR_PDS_FORCE_MM_ISO_EN_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_iso_en(&mut self) -> CR_PDS_FORCE_USB_ISO_EN_W<7> {
        CR_PDS_FORCE_USB_ISO_EN_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_pds_rst(&mut self) -> CR_PDS_FORCE_NP_PDS_RST_W<8> {
        CR_PDS_FORCE_NP_PDS_RST_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_mm_pds_rst(&mut self) -> CR_PDS_FORCE_MM_PDS_RST_W<9> {
        CR_PDS_FORCE_MM_PDS_RST_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_pds_rst(&mut self) -> CR_PDS_FORCE_WB_PDS_RST_W<10> {
        CR_PDS_FORCE_WB_PDS_RST_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_pds_rst(&mut self) -> CR_PDS_FORCE_USB_PDS_RST_W<11> {
        CR_PDS_FORCE_USB_PDS_RST_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_mem_stby(&mut self) -> CR_PDS_FORCE_NP_MEM_STBY_W<12> {
        CR_PDS_FORCE_NP_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_mm_mem_stby(&mut self) -> CR_PDS_FORCE_MM_MEM_STBY_W<13> {
        CR_PDS_FORCE_MM_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_mem_stby(&mut self) -> CR_PDS_FORCE_WB_MEM_STBY_W<14> {
        CR_PDS_FORCE_WB_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_mem_stby(&mut self) -> CR_PDS_FORCE_USB_MEM_STBY_W<15> {
        CR_PDS_FORCE_USB_MEM_STBY_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_np_gate_clk(&mut self) -> CR_PDS_FORCE_NP_GATE_CLK_W<16> {
        CR_PDS_FORCE_NP_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_mm_gate_clk(&mut self) -> CR_PDS_FORCE_MM_GATE_CLK_W<17> {
        CR_PDS_FORCE_MM_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_wb_gate_clk(&mut self) -> CR_PDS_FORCE_WB_GATE_CLK_W<18> {
        CR_PDS_FORCE_WB_GATE_CLK_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn cr_pds_force_usb_gate_clk(&mut self) -> CR_PDS_FORCE_USB_GATE_CLK_W<19> {
        CR_PDS_FORCE_USB_GATE_CLK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDS_CTL2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pds_ctl2](index.html) module"]
pub struct PDS_CTL2_SPEC;
impl crate::RegisterSpec for PDS_CTL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pds_ctl2::R](R) reader structure"]
impl crate::Readable for PDS_CTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pds_ctl2::W](W) writer structure"]
impl crate::Writable for PDS_CTL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pds_ctl2 to value 0x0002_2222"]
impl crate::Resettable for PDS_CTL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_2222;
}
